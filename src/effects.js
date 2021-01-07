export const absFade = (node, {delay = 0, duration = 180}) => {
    const o = getComputedStyle(node).opacity

    return {
        delay,
        duration,
        css: t => `
            opacity: ${t * o};
            position: absolute;    
        `
    }
}

export const fade = (node, {delay = 0, duration = 180}) => {
    const o = getComputedStyle(node).opacity

    return {
        delay,
        duration,
        css: t => `opacity: ${t * o};`
    }
}