declare global {
    interface ArrayConstructor {
        repeat(what: any, l: number): any[];
    }
}

Array.repeat = (what: any, l: number) => {
    let arr = new Array(l)
    while(l) arr[--l] = what
    return arr
}

export {}
