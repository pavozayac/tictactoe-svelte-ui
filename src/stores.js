import {writable} from 'svelte/store'

export const scene = writable('board')
export const size = writable(3)