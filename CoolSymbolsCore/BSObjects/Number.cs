﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace BeautifulSymbols.BSObjects
{
    public class Number : BSObject
    {
        public Number(double number) => Value = number;

        public double Value;
        public override string StackView() => Value.ToString();

        public static implicit operator Number(double v) => new Number(v);
        public override BSObject Clone() => new Number(Value);
    }
}