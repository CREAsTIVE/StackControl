﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace StackControl.SCCommands
{
    internal class ListGenerator : BuiltInCommand
    {
        public override void Call(RuntimeEnvironment environment)
        {
            BSObjects.SCArray array = new();
            var current = environment.GetCurrent();
            while (!(current is BSObjects.SCListOpener))
            {
                array.Values.Add(current);
                environment.Pop();
                current = environment.GetCurrent();
            }
            environment.Pop();
            array.Values.Reverse();
            environment.Push(array);
        }
    }
}
