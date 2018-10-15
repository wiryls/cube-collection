using System.Collections.Generic;

namespace Editor.Models.Cube
{
    public class Path
    {
        public bool       Loop = false;
        public List<Pair> Move = new List<Pair>();

        public bool None => Move.Count == 0;

        public struct Pair
        {
            public int   Count;
            public Doing Doing;
        }

        public enum Doing
        {
            Idle,
            Left,
            Down,
            Up,
            Right,
        }
    }
}
