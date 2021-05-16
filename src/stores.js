import {writable} from 'svelte/store'

export const scene = writable('start')
export const loading = writable(true)
export const size = writable(0)
export const board = writable(null)
export const mode = writable()
export const nameX = writable()
export const nameO = writable()
export const winner = writable('none')
export const nodes = writable(0)