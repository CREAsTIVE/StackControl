﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace BeautifulSymbols.BSCommands
{
    public class SwapDouble : BuiltInFunction
    {
        public override void Call(RuntimeEnvironment environment)
        {
            var a = environment.Pop();
            var b = environment.Pop();
            environment.Push(a);
            environment.Push(b);
        }
    }
}
