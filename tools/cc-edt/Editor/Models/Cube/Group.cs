using System.Collections.Generic;

namespace Editor.Models.Cube
{
    public class Group
    {
        private readonly LinkedList<Unit> body;
        public Type Type { get; set; }
        public Path Path { get; }

        public Group(Type type)
        {
            Type = type;
            Path = new Path();
            body = new LinkedList<Unit>();
        }

        public IEnumerable<Unit> Body => body;

        internal bool Detach(Unit unit)
        {
            if (this != unit.Group)
                return false;

            body.Remove(unit);
            return true;
        }

        internal bool Attach(Unit unit)
        {
            if (this == unit.Group)
                return false;

            body.AddFirst(unit);
            return true;
        }
    }
}
