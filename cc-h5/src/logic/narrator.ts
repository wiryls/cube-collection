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
    private static readonly RESOURCE_NAME: string = "level_index";
    private static readonly   MEMORY_NAME: string = "data";

    private readonly node = new Array<Narrator.Node>();
    private readonly data = Narrator.Script.Default();

    constructor(private readonly load: ILoader, private readonly save: ISaver)
    {
        if (load.has(Narrator.RESOURCE_NAME)) {
            const raw = load.get(Narrator.RESOURCE_NAME) as Array<Narrator.Node>;
            if (Array.isArray(raw) === false) {
                throw new Error(`Narrator: Invalid Map ${raw}`);
            } else if (raw.some(n => Guard.isNode(n) === false)) {
                throw new Error(`Narrator: Invalid Map ${raw}`);
            } else {
                 this.node.push(...raw);
            }
        }

        if (save.has(Narrator.MEMORY_NAME)) {
            const raw = save.get(Narrator.MEMORY_NAME) as Narrator.Script;
            if (Guard.isScript(raw)) {
                this.data = raw;
            } else {
                throw new Error(`Narrator: Invalid Data ${raw}`);
            }
        } else if (this.node.length > 0) {
            this.data.milestone = this.node[0].name;
        } else {
            throw new Error("Narrator: Data not exists");
        }
    }

    zero(): undefined|Seed
    {
        if (this.node.length < 0)
            throw new Error("Narrator: Map is empty");

        this.data.milestone = this.node[0].name;
        return this.tell();
    }

    tell(): undefined|Seed
    {
        return this.seed(this.data.milestone);
    }

    next(commit: ReadonlyArray<number>): undefined|Seed
    {
        const pos = this.data.milestone;
        const now = this.node.find(n => n.name === pos);
        if (now === undefined)
            throw new Error(`Narrator: Cannot find Node ${pos}`);

        let cas = now.case.find(c => Narrator.match(c.cond, commit));
        if (cas === undefined)
            cas = now.case.find(c => c.cond === undefined);
        if (cas === undefined)
            throw new Error(`Narrator: Missing Defualt at ${pos}`);

        this.data.milestone = cas.next;
        return this.seed(cas.next);
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
        if (Array.isArray(lhs) === false || Array.isArray(rhs) === false)
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
            && Array.isArray(its.case)
            && its.case.every(c => isCase(c))
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