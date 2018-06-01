namespace core {
/////////////////////////////////////////////////////////////////////////////

export namespace Transform
{
    import Action = Cube.Action;
    import Status = Cube.Status;

    /////////////////////////////////////////////////////////////////////////

    export function link(cs: ReadonlyArray<ICube>, width: number, height: number): void
    {
        // Grid is good.
        // WARNING: all of these become invalid when cs changes,
        //          do NOT modify cs until these are not needed.
        const link = cs.filter(c => c.live && c.active).map((c, i) => new Link(c, i));

        const dset = new DisjointSet(link);
        const item = new ItemGrid<Link>(width, height);
        const edge = new EdgeGrid(width, height);
        for (const l of link) {
            item.set(l.cube.entity, l);
            edge.put(l.cube.entity);
        }

        // find and mark linked.
        link.forEach(l =>
            item.get(edge.out(l.cube.entity)).filter(
            r   => r.cube.active
                && r.cube.absorbable(l.cube) === false
                && l.cube.absorbable(r.cube) === true
            ).forEach(o =>
                dset.join(l.index, o.index)
            )
        );

        // merge connected
        for(const foe of dset.groups())
        // NOTE: item and edge become invalid after this line.
            merge(foe.map(l => l.cube));        
    }

    class Link
    {
        constructor(
            public readonly  cube: ICube,
            public readonly index: number)
        {}
    }

    /////////////////////////////////////////////////////////////////////////

    export function move(cs: ReadonlyArray<ICube>, width: number, height: number): void
    {
        // Grid is good.
        // WARNING: all of these become invalid when cs changes,
        //          do NOT modify cs until these are not needed.
        const item = new ItemGrid<Node>(width, height);
        const edge = new EdgeGrid(width, height);
        
        const node = cs.filter(c => c.live).map((c, i) => new Node(c, i));
        const dset = new DisjointSet(node);

        const next = node.filter(n => n.cube.moving);
        const move = next.filter(n => n.cube.action !== Action.Idle);

        for (const n of node)
            item.set(n.cube.entity, n);
        for (const n of move)
            edge.put(n.cube.entity);

        // find stop
        const seed = new Array<Node>();
        for(const n of move) {
            const act = n.cube.action;   
            const nxt = edge.out(n.cube.entity, act);
            const foe = item.get(nxt);

            const dif = foe.filter(o => o.cube.action !== act);
            if (dif.length > 0 || item.has(nxt) === false) {
                // stop by other cubes or by wall
                if (n.cube.active)
                    dif .filter (o => o.cube.active)
                        .filter (o => o.cube.absorbable(n.cube) || n.cube.absorbable(o.cube))
                        .forEach(o => dset.join(n.index, o.index));

                // mark as stop
                n.cube.status = Status.Stop;
                seed.push(n);
            } else {
                // maybe not stop
                foe .filter (o => o.cube.action === act)
                    .forEach(o => o.node.push(n));
            }
        }

        // find lock
        for (const n of move) {
            if (n.cube.status !== Status.Free)
                continue; // ignore non-free

            for(const v of edge.out(n.cube.entity, n.cube.action)) {
                const foe = Action.Move
                    .map(a => {
                        const o = item.get(Action.toVec2(a).plus(v));
                        return(o === undefined
                            || o.cube.status === Status.Stop
                            || o.cube.action !== Action.opposite(a)
                            )? undefined : o; })
                    .filter((o): o is Node => o !== undefined)
                    .map   (o => o as Node)
                    ;

                const one = foe.find(n => n.cube.active && foe.every(o =>
                    o === n ||
                    o.cube.active &&
                    o.cube.absorbable(n.cube) === false &&
                    n.cube.absorbable(o.cube) === true
                ));

                for (const o of foe.filter(o => o !== one)) {
                    o.cube.status = Status.Lock;
                    seed.push(o);
                }
            }
        }

        // solve
        for(const src of seed) {
            const act = Action.opposite(src.cube.action);
            for(const que = new Array<Node>(src); que.length != 0; ) {
                const n = que.shift();
                if (n === undefined)
                    break;
                
                if (n.cube.active)
                    n.node
                        .filter (o => o.cube.active)
                        .filter (o => o.cube.absorbable(n.cube) || n.cube.absorbable(o.cube))
                        .forEach(o => dset.join(n.index, o.index));
                        ;

                for (const o of n.node.filter(o => o.cube.status < n.cube.status)) {
                    o.cube.status = o.cube.status;
                    que.push(o);
                }
            }
        }

        // merge groups.
        for(const foe of dset.groups())
        // NOTE: item and edge become invalid from this line.
            merge(foe.map(n => n.cube));
    }

    class Node
    {
        readonly node: Array<Node> = new Array<Node>();
        constructor(
            readonly  cube: ICube,
            readonly index: number)
        { }
    }

    /////////////////////////////////////////////////////////////////////////

    function merge(arena: ReadonlyArray<ICube>): void
    {
        if (arena.length <= 1)
            return;

        const winner = arena.find(c => arena.every(o => c === o || c.absorbable(o)));
        if (winner !== undefined)
            winner.absorb(arena.filter(o => winner !== o));
    }

    /////////////////////////////////////////////////////////////////////////
}

/////////////////////////////////////////////////////////////////////////////
}