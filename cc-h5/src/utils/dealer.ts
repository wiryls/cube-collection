namespace utils {
/////////////////////////////////////////////////////////////////////////////

export class Loader implements logic.ILoader
{
    constructor() { }

    has(key: string): boolean
    {
         return RES.hasRes(key)
    }

    get(key: string): any
    {
        return RES.getRes(key);
    }
}

export class Saver implements logic.ISaver
{
    constructor() { }

    has(key: string): boolean
    {
        return egret.localStorage.getItem(key) !== null;
    }

    get(key: string): any
    {
        return JSON.parse(egret.localStorage.getItem(key));
    }

    set(key: string, val: any): void
    {
        egret.localStorage.setItem(key, JSON.stringify(val));
    }
}

/////////////////////////////////////////////////////////////////////////////
}