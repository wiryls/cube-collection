using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;
using System;
using System.Linq;

namespace Editor.Models.Utilities
{
    class ObjectToArrayConverter<T> : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return typeof(T) == objectType;
        }

        static bool IsIgnored(JsonProperty property)
        {
            return property.Ignored || !property.Readable || !property.Writable;
        }

        public override void WriteJson(JsonWriter writer, object val, JsonSerializer ser)
        {
            var type = val.GetType();
            var resolver = ser.ContractResolver;
            if (!(resolver.ResolveContract(type) is JsonObjectContract contract))
                throw new JsonSerializationException("invalid type " + type.FullName);

            var list = contract.Properties
                .Where(p => !IsIgnored(p))
                .Select(p => p.ValueProvider.GetValue(val))
                ;

            ser.Serialize(writer, list);
        }

        public override object ReadJson(JsonReader reader, Type type, object val, JsonSerializer ser)
        {
            if (reader.TokenType == JsonToken.Null)
                return null;

            var token = JArray.Load(reader);
            var resolver = ser.ContractResolver;
            if (!(resolver.ResolveContract(type) is JsonObjectContract contract))
                throw new JsonSerializationException("invalid type " + type.FullName);

            var value = val ?? contract.DefaultCreator();
            foreach (var pair in contract.Properties
                .Where(p => !IsIgnored(p))
                .Zip(token, (p, v) => new { Value = v, Property = p }))
            {
                var propertyValue = pair.Value.ToObject(pair.Property.PropertyType, ser);
                pair.Property.ValueProvider.SetValue(value, propertyValue);
            }

            return value;
        }
    }

    // Note:
    // [C# JSON.NET - Deserialize response that uses an unusual data structure]
    // (https://stackoverflow.com/a/39462464)
}
