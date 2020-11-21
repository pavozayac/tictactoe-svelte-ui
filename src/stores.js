import {writable} from 'svelte/store'

export const scene = writable('start')
export const size = writable(0)
export const board = writable(null)