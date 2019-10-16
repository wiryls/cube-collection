using System;
using System.Collections.Generic;
using System.Linq;

namespace Editor.Models
{
    public class World
    {
        private readonly Cube.Grid                  grid;
        private readonly Dictionary<string, string> head;
        private readonly List<Seed.Vec2>            dest;

        public World()
            : this(4, 4)
        { }

        public World(int width, int height)
        {
            grid = new Cube.Grid(width, height);
            head = new Dictionary<string, string>();
            dest = new List<Seed.Vec2>();
        }

        public World(Seed seed)
        {
            head = new Dictionary<string, string>(seed.head);
            grid = new Cube.Grid(seed.size.width, seed.size.height);
            dest = seed.dest.ToList();

            foreach(var c in seed.cube.Where(c => c.body.Count > 0))
            {
                var vec = c.body[0];
                grid.Place((Cube.Type)c.type, vec.x, vec.y);
                foreach (var one in c.body)
                    grid.Spread(vec.x, vec.y, one.x, one.y);

                var unit = grid.Get(vec.x, vec.y);
                if (unit == null)
                    continue;

                var group = unit.Group;
                if (group == null)
                    continue;

                group.Path.Loop = c.move.loop;
                group.Path.Move = c.move.path.Select(pair => new Cube.Path.Pair
                {
                    Count = pair.time,
                    Doing = (Cube.Path.Doing)pair.type
                }).ToList();
            }
        }

        public void Expand(int l, int b, int t, int r)
        {
            // keep in range
            l = Math.Max(l, 1 - grid.Width);
            t = Math.Max(t, 1 - grid.Height);

            // expand grid
            grid.Expand(l, b, t, r);

            // expand dest
            foreach (var v in dest)
            {
                v.x += l;
                v.y += t;
            }
            dest.RemoveAll(v => grid.Has(v.x, v.y) == false);
        }

        public void Add()
        {

        }
    }
}
