namespace logic {
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
        for (const l of link) {
            for(const que = new Array<Link>(l); que.length !== 0; ) {
                const n = que.shift();
                if (n === undefined)
                    break;

                item.get(edge.out(n.cube.entity)).filter(
                r   => r.cube.active
                    && r.cube.absorbable(l.cube) === false
                    && l.cube.absorbable(r.cube) === true
                    && dset.same(l, r) === false
                ).forEach(o => {
                    dset.join(l.index, o.index)
                    que.push(o);
                });
            }
        }

        // merge connected.
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

        const item = new ItemGrid<Node>(width, height); // grid of cubes
        const edge = new EdgeGrid(width, height);       // grid of edges
        
        const node = cs.filter(c => c.live).map((c, i) => new Node(c, i));
        const dset = new DisjointSet(node);

        const next = node.filter(n => n.cube.moving);
        const move = next.filter(n => n.cube.action !== Action.Idle);

        for (const n of node)
            item.set(n.cube.entity, n);
        for (const n of move)
            edge.put(n.cube.entity);

        // find stopped.
        // no entry.
        const seed = new Array<Node>();
        for(const n of move) {
            const act = n.cube.action;   
            const nxt = edge.out(n.cube.entity, act);
            const foe = item.get(nxt);

            const dif = foe.filter(o => o.cube.action !== act);
            if (dif.length > 0 || item.has(nxt) === false) {
                // stopped by other cubes or by wall
                if (n.cube.active)
                    dif .filter (o => o.cube.active)
                        .filter (o => o.cube.absorbable(n.cube) || n.cube.absorbable(o.cube))
                        .forEach(o => dset.join(n.index, o.index));

                // mark as stopped.
                n.cube.status = Status.Stop;
                seed.push(n);
            } else {
                // maybe locked.
                foe.forEach(o => o.node.push(n));
            }
        }

        // find locked.
        // multiple cubes compete for the same location.
        for (const n of move.filter(n => n.cube.status === Status.Free)) {
            for(const v of edge.out(n.cube.entity, n.cube.action)) {
                const foe = Action.Move
                    .map(a => {
                        const  o = item.get(Action.toVec2(a).plus(v));
                        return(o !== undefined
                            && o.cube.status !== Status.Stop
                            && o.cube.action === Action.opposite(a)
                            )? o : undefined; })
                    .filter((o): o is Node => o !== undefined)
                    .map   (o => o as Node)
                    ;

                if (foe.length > 1) {
                    const one = foe.find(n => n.cube.active && foe.every(o =>
                        o === n || (
                        o.cube.active &&
                        o.cube.absorbable(n.cube) === false &&
                        n.cube.absorbable(o.cube) === true)
                    ));

                    for (const o of foe.filter(o => o !== one)) {
                        o.cube.status = Status.Lock;
                        seed.push(o);
                    }
                }
            }
        }

        // find blocked.
        // cube will be absored halfway.
        for (const n of move.filter(n => n.cube.status === Status.Free && edge
            .cor(n.cube.entity, n.cube.action)
            .map   (v => item.get(v))
            .filter((o): o is Node => o !== undefined)
            .map   (o => o as Node)
            .some  (o => o.cube.status === Status.Free
                      && o.cube.action === Action.opposite(n.cube.action)
                      && o.cube.absorbable(n.cube))
        )) {
            n.cube.status = Status.Lock;
            seed.push(n);
        }

        // solve dependent.
        for(const src of seed) {
            for(const que = new Array<Node>(src); que.length !== 0; ) {
                const n = que.shift();
                if (n === undefined)
                    break;
                
                if (n.cube.active)
                    n.node
                        .filter (o => o.cube.active)
                        .filter (o => o.cube.absorbable(n.cube) || n.cube.absorbable(o.cube))
                        .forEach(o => dset.join(n.index, o.index))
                        ;

                for (const o of n.node.filter(o => o.cube.status < n.cube.status)) {
                    o.cube.status = n.cube.status;
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