using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using Platform.Converters;
using Platform.Data.Doublets.Decorators;
using Platform.Numbers;
using Platform.Reflection;
using Platform.Unsafe;


namespace Platform.Data.Doublets.Numbers.Raw
{
    public class BigIntegerToRawNumberSequenceConverter<TLink> : LinksDecoratorBase<TLink>, IConverter<BigInteger, TLink>
    where TLink : struct
    {
        public readonly EqualityComparer<TLink> EqualityComparer = EqualityComparer<TLink>.Default;
        private readonly IConverter<TLink> _addressToNumberConverter;
        public static readonly int BitsStorableInRawNumber = Structure<TLink>.Size - 1;
        private static readonly int _bitsPerRawNumber = NumericType<TLink>.BitsSize - 1;
        private static readonly TLink _maximumValue = NumericType<TLink>.MaxValue;
        private static readonly TLink _bitMask = Bit.ShiftRight(_maximumValue, 1);
        
        public BigIntegerToRawNumberSequenceConverter(ILinks<TLink> links, IConverter<TLink> addressToNumberConverter) : base(links)
        {
            _addressToNumberConverter = addressToNumberConverter;   
        }

        private Stack<TLink> GetRawNumberParts(BigInteger bigInteger)
        {
            Stack<TLink> rawNumbers = new();
            for (BigInteger currentBigInt = bigInteger; currentBigInt > 0; currentBigInt >>= 63)
            {
                var bigIntBytes = currentBigInt.ToByteArray();
                var bigIntWithBitMask = Bit.And(bigIntBytes.ToStructure<TLink>(), _bitMask);
                var rawNumber = _addressToNumberConverter.Convert(bigIntWithBitMask);
                rawNumbers.Push(rawNumber);
            }
            return rawNumbers;
        }

        public TLink Convert(BigInteger bigInteger)
        {
            var rawNumbers = GetRawNumberParts(bigInteger);
            TLink part = rawNumbers.First();
            foreach (var rawNumber in rawNumbers.Skip(1))
            {
                part = _links.GetOrCreate(rawNumber, part);
            }
            return part;
        }
    }
}