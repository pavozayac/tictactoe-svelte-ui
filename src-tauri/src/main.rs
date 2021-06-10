#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod board;

use std::sync::{Arc, Mutex};

use board::*;
mod files;
use tauri;

struct Data {
  pub board: Board,
  pub config: files::Config
}

type State = Arc<Mutex<Data>>;


#[tauri::command]
fn load (window: tauri::Window, state: tauri::State<State>) {
  let one = state.lock().unwrap().config.player_one.clone();
  let two = state.lock().unwrap().config.player_two.clone();

  window.eval(&format!("app.setNames('{}', '{}')", one, two)).unwrap();
}

#[tauri::command]
fn test_size(size: usize) {
  println!("{}", size);
}

#[tauri::command]
fn init (size: usize, player_one: String, player_two: String, state: tauri::State<State>) {
  state.lock().unwrap().config.player_one = player_one;
  state.lock().unwrap().config.player_two = player_two;

  state.lock().unwrap().config.save().unwrap();

  state.lock().unwrap().board = Board::new(size);
}

#[tauri::command]
fn player_move (x: usize, y: usize, state: tauri::State<State>, window: tauri::Window) {
  state.lock().unwrap().board.make_move((y, x)).unwrap();
  let jsonable = state.lock().unwrap().board.array.jsonable();

  if let Some(sign) = state.lock().unwrap().board.winner_sign() {

      window.eval(&format!("app.refreshBoard('{}')", &serde_json::to_string(&jsonable).unwrap())).expect("something wrong with refreshboard");

      window.eval(&format!(
          "app.gameEnd(\"{}\")",
          match sign {
              FieldState::Circle => "o",
              FieldState::Cross => "x",
              FieldState::Blank => "tie",
          }
      )).unwrap();
  } else {
      window.eval(&format!("app.refreshBoard('{}')", &serde_json::to_string(&jsonable).unwrap())).expect("Something wrong with refreshboard");
  }

}

#[tauri::command]
fn computer(state: tauri::State<State>, window: tauri::Window) {

  let blanks = state.lock().unwrap().board.blank_fields();

  let jsonable = state.lock().unwrap().board.array.jsonable();

  if !blanks.is_empty() {
      let mut board = state.lock().unwrap().board;
      let depth = state.lock().unwrap().config.depth;


      let coords = computer_move(&mut board, depth);
      state.lock().unwrap().board.make_move(coords).unwrap();

      if let Some(sign) = state.lock().unwrap().board.winner_sign() {

          window.eval(&format!("app.refreshBoard('{}')", &serde_json::to_string(&jsonable).unwrap())).unwrap();
          
          window.eval(&format!(
              "app.gameEnd(\"{}\")",
              match sign {
                  FieldState::Circle => "o",
                  FieldState::Cross => "x",
                  FieldState::Blank => "tie",
              }
          )).unwrap();
      } else {
          window.eval(&format!("app.refreshBoard('{}')", &serde_json::to_string(&jsonable).unwrap())).unwrap();
      }
  }
}

#[tauri::command]
fn refresh (state: tauri::State<State>, window: tauri::Window) {
  let jsonable = state.lock().unwrap().board.array.jsonable();

  window.eval(&format!("app.refreshBoard('{}')", &serde_json::to_string(&jsonable).unwrap())).unwrap();
}

#[tauri::command]
fn exit(window: tauri::Window) {
  window.close().unwrap();
}

fn main() {
  tauri::Builder::default()
    .manage(Arc::new(Mutex::new(Data {
      board: Board::new(0),
      config: files::Config::from_file()
    })))
    .invoke_handler(tauri::generate_handler![load, test_size, init, player_move, computer, refresh, exit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
