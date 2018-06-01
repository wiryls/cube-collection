namespace logic {
/////////////////////////////////////////////////////////////////////////////

export class DisjointSet<T> {
    private readonly parents: Array <number>;
    private readonly indexes: Map<T, number>;

    constructor(private readonly sources : ReadonlyArray <T>) {
        this.indexes = new Map(this.sources.map((v, i) => [v, i] as [T, number]));
        this.parents = this.sources.map((_, i) => i);
    }

    same(lhs: T, rhs: T): boolean;
    same(lhs: number, rhs: number): boolean;
    same(lhs: T|number, rhs: T|number): boolean {
        const l = this.root(lhs);
        const r = this.root(rhs);
        if (l === undefined || r === undefined)
            return false;

        return l === r;
    }

    join(lhs: T, rhs: T): boolean;
    join(lhs: number, rhs: number): boolean;
    join(lhs: T|number, rhs: T|number): boolean {
        const l = this.root(lhs);
        const r = this.root(rhs);
        if (l === undefined || r === undefined)
            return false;

        if (l !== r)
            this.parents[r] = this.parents[l];

        return true;
    }

    root(src: T|number): number|undefined {
        const index = this.index(src);
        if (index === undefined || index >= this.parents.length)
            return undefined;

        let result = index;
        while(this.parents[result] != result)
            result = this.parents[result];

        for (let i = index; i !== result; )
            [i, this.parents[i]] = [this.parents[i], result];

        return result;
    }

    groups(): Array<Array<T>> {
        const ret  = new Array<Array<T>>(this.parents.length);

        for (let i = 0; i < this.parents.length; i++) {
            const j = this.root(i);
            if (j === undefined || j === i || this.sources[j] === undefined)
                continue;

            if (ret[j] === undefined)
                ret[j] = new Array<T>(this.sources[j]);
            
            ret[j].push(this.sources[i]);
        }

        return ret.filter(a => a !== undefined);
    }

    alones(): Array<T> {
        return this.sources.filter((_, i) => this.parents[i] === i);
    }

    private index(src: number|T): number|undefined {
        if (typeof src !== 'number')
            return this.indexes.get(src);
        else if (src > this.parents.length)
            return undefined;
        else
            return src;
    }
}

/////////////////////////////////////////////////////////////////////////////
}