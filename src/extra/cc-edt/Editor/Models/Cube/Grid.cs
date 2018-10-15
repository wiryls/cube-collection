using System;
using System.Linq;

namespace Editor.Models.Cube
{
    public class Grid
    {
        private Unit[,] data;

        public Grid()
            : this(1, 1)
        { }

        public Grid(int width, int height)
        {
            width  = Math.Max(1, width);
            height = Math.Max(1, height);

            data = new Unit[width, height];
        }

        public int Width
            => data.GetLength(0)
            ;

        public int Height
            => data.GetLength(1)
            ;

        public bool Has(int x, int y)
            => x > 0 && x < Width
            && y > 0 && y < Height
            ;

        public Unit Get(int x, int y)
            => Has (x, y)
            ?  data[x, y]
            :  null
            ;

        /////////////////////////////////////////////////////////////////////

        public bool Place(Type type, int x, int y)
        {
            if (Has(x, y) == false)
                return false;

            if (data[x, y] == null)
                data[x, y] = new Unit(new Group(type), x, y);
            else
                data[x, y].Group.Type = type;

            return true;
        }

        public bool Spread(int x0, int y0, int x1, int y1)
        {
            if (Has(x0, y0) == false ||
                Has(x1, y1) == false ||
                (x0 == x1 && y0 == y1))
                return false;

            var source = data[x0, y0];
            if (source == null)
                Erase(x1, y1);
            else if (data[x1, y1] == null)
                data[x1, y1] = new Unit(source.Group, x1, y1);
            else
                data[x1, y1].Transfer(source.Group);

            return true;
        }

        public bool Erase(int x, int y)
        {
            if (Has(x, y) == false)
                return false;

            if (data[x, y] == null)
                return false;

            data[x, y].Destruct();
            data[x, y] = null;
            return true;
        }

        public bool Move(Group group, int dx, int dy)
        {
            if (group.Body.Any(u => !Has(u.X + dx, u.Y + dy)))
                return false;

            if (group.Body.Select(u => data[u.X + dx, u.Y + dy]).Any(u => u != null && u.Group != group))
                return false;

            foreach (var unit in group.Body)
            {
                data[unit.X, unit.Y] = null;
                unit.X += dx;
                unit.Y += dy;
                data[unit.X, unit.Y] = unit;
            }
            return true;
        }

        /////////////////////////////////////////////////////////////////////

        public void Expand(int l, int b, int t, int r)
        {
            var wid = Width;
            var hgt = Height;

            l = Math.Max(l, 1 - wid);
            r = Math.Max(r, 1 - wid - l);
            t = Math.Max(t, 1 - hgt);
            b = Math.Max(b, 1 - hgt - t);

            wid += l + r;
            hgt += b + t;

            var src = data;
            var dst = new Unit[wid, hgt];
            data = dst;

            for (int x = 0, u = src.GetLength(0); x < u; x++)
                for (int y = 0, v = src.GetLength(1); y < v; y++)
                {
                    var unit = src[x, y];
                    unit.X += l;
                    unit.Y += t;

                    if (Has(unit.X, unit.Y))
                        dst[unit.X, unit.Y] = unit;
                    else
                        unit.Destruct();
                }
        }

        public void Clear()
        {
            for (int x = 0, m = data.GetLength(0); x < m; x++)
                for (int y = 0, n = data.GetLength(1); y < n; y++)
                    data[x, y] = null;
        }
    }
}
