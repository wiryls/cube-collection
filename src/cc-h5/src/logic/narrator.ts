namespace logic {
/////////////////////////////////////////////////////////////////////////////

export interface ILoader
{
    has(key: string): boolean;
    get(key: string): any;
}

export interface ISaver
{
    has(key: string): boolean;
    get(key: string): any;
    set(key: string, val: any): void;
}

/////////////////////////////////////////////////////////////////////////////

export class Narrator
{
    private static readonly RESOURCE_NAME: string = "maps";
    private static readonly   MEMORY_NAME: string = "data";

    private readonly node = new Array<Narrator.Node>();
    private readonly data = Narrator.Script.Default();

    constructor(private readonly load: ILoader, private readonly save: ISaver)
    {
        if (load.has(Narrator.RESOURCE_NAME)) {
            const raw = load.get(Narrator.RESOURCE_NAME) as Array<Narrator.Node>;
            if (Array.isArray(raw) === false) {
                console.error("Narrator: Invalid Map", raw);
            } else if (raw.some(n => Guard.isNode(n) === false)) {
                console.error("Narrator: Invalid Map", raw);
            } else {
                 this.node.push(...raw);
            }
        }

        if (save.has(Narrator.MEMORY_NAME)) {
            const raw = save.get(Narrator.MEMORY_NAME) as Narrator.Script;
            if (Guard.isScript(raw)) {
                this.data = raw;
            } else {
                console.error("Narrator: Invalid Data", raw);
            }
        } else if (this.node.length > 0) {
            this.data.milestone = this.node[0].name;
        } else {
            console.error("Narrator: Data not exists");
        }
    }

    tell(): undefined|Seed
    {
        return this.seed(this.data.milestone);
    }

    next(commit: ReadonlyArray<number>): undefined|Seed
    {
        const pos = this.data.milestone;
        const now = this.node.find(n => n.name === pos);
        if (now === undefined) {
            console.error("Narrator: Cannot find Node", pos);
            return undefined;
        }

        let nxt = now.case.find(c => Narrator.match(c.cond, commit));
        if (nxt === undefined)
            nxt = now.case.find(c => c.cond === undefined);
        if (nxt === undefined) {
            console.error("Narrator: Missing Defualt at", pos);
            return undefined;
        }

        this.data.milestone = nxt.next;
        return this.seed(nxt.next);
    }

    private seed(key: string): undefined|Seed
    {
        if (this.load.has(key) === false)
            return undefined;
        
        const seed = this.load.get(key);
        if (!Guard.isSeed(seed)) {
            console.error("Narrator: Invalid Seed", key, "With", seed);
            return undefined;
        }

        return seed;
    }

    private static match(
        lhs: ReadonlyArray<number>|undefined,
        rhs: ReadonlyArray<number>|undefined): boolean
    {
        if (lhs === undefined || rhs === undefined)
            return false;
        if (Array.isArray(lhs) === false || Array.isArray(rhs))
            return false;
        
        const n = Math.min(lhs.length, rhs.length);
        for (let i = 0; i < n; i++)
            if (lhs[i] !== rhs[i])
                return false;

        return true;
    }
}

export namespace Narrator
{
    export interface Node
    {
        name: string;
        case: Array<Case>;
    }

    export interface Case
    {
        cond: Cond|undefined;
        next: string;
    }

    export interface Cond extends Array<number>
    {

    }

    export interface Script
    {
        milestone: string;
    }

    export namespace Script
    {
        export function Default(): Script
        {
            return {
                milestone: ""
            };
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

export namespace Guard
{
    import Node = Narrator.Node;
    import Case = Narrator.Case;
    import Cond = Narrator.Cond;

    export function isNode(it: any): it is Node
    {
        const its = it as Node;
        return typeof its.name === 'string'
            && isCase(its.case)
            ;
    }

    function isCase(it: any): it is Case
    {
        const its = it as Case;
        return typeof its.next === 'string';
    }

    import Script = Narrator.Script;

    export function isScript(it: any): it is Script
    {
        const its = it as Script;
        return typeof its.milestone === 'string'
            ;
    }
}

/////////////////////////////////////////////////////////////////////////////
}