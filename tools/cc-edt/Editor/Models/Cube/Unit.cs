using System.Collections.Generic;

namespace Editor.Models.Cube
{
    public class Unit
    {
        public Unit(Group group, int x, int y)
        {
            Group = group;
            X = x;
            Y = y;
        }

        public Group Group { get; private set; }
        public int Y { get; set; }
        public int X { get; set; }

        public Type              Type => Group.Type;
        public IEnumerable<Unit> Ally => Group.Body;

        public bool Transfer(Group other)
        {
            if (Group == other || other == null)
                return false;

            if (Group != null)
                Group.Detach(this);

            if (other != null)
                other.Attach(this);

            Group = other;
            return true;
        }

        internal bool Destruct()
        {
            if (Group == null)
                return false;

            Group.Detach(this);
            Group = null;
            return true;
        }
    }
}
