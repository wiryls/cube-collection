using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Editor.Models
{
    using Utilities;

    public class Seed
    {
        public Head head;
        public Size size;
        public List<Cube> cube;
        public List<Vec2> dest;

        public class Head : Dictionary<string, string>
        {

        }

        public struct Size
        {
            public int width;
            public int height;
        }

        public class Cube
        {
            [JsonConverter(typeof(StringEnumConverter))]
            public Type type;

            public List<Vec2> body;

            [JsonProperty(NullValueHandling = NullValueHandling.Ignore)]
            public Move move;

            [JsonConverter(typeof(StringEnumConverter))]
            public enum Type
            {
                W = Models.Cube.Type.White,
                G = Models.Cube.Type.Green,
                B = Models.Cube.Type.Blue,
                R = Models.Cube.Type.Red,
            }

            public class Move
            {
                public bool loop;
                public Path path;

                public class Path : List<Pair>
                {

                }

                [JsonConverter(typeof(ObjectToArrayConverter<Pair>))]
                public struct Pair
                {
                    [JsonProperty(Order = 1)]
                    [JsonConverter(typeof(StringEnumConverter))]
                    public Type type;

                    [JsonProperty(Order = 2)]
                    public int time;

                    [JsonConverter(typeof(StringEnumConverter))]
                    public enum Type
                    {
                        I = Models.Cube.Path.Doing.Idle,
                        L = Models.Cube.Path.Doing.Left,
                        D = Models.Cube.Path.Doing.Down,
                        U = Models.Cube.Path.Doing.Up,
                        R = Models.Cube.Path.Doing.Right,
                    }
                }
            }
        }

        [JsonConverter(typeof(ObjectToArrayConverter<Vec2>))]
        public class Vec2
        {
            [JsonProperty(Order = 1)] public int x;
            [JsonProperty(Order = 2)] public int y;
        }
    }

    // Note:
    //
    // [Json.Net - Introduction]
    // (https://www.newtonsoft.com/json/help/html/Introduction.htm)
    // [Deserialize json tuple to object using ordinal index]
    // (https://stackoverflow.com/a/47963611)
    // [Ignoring null fields in Json.net]
    // (https://stackoverflow.com/a/22039616)
}
