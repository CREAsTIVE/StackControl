﻿using StackControl;
using StackControl.SCObjects;
using StackControl.SCCommands;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace StackControle.SCCommands
{
    public class Maximum : BuiltInCommand
    {
        public override void Call(RuntimeEnvironment environment) =>
            environment.Push(environment.Pop().As<SCArray>().Values.MaxBy(e => e.As<SCNumber>().Value)?.As<SCNumber>() ?? new SCNumber(double.MinValue));
    }
	public class ArrayLength : BuiltInCommand
	{
		public override void Call(RuntimeEnvironment environment) =>
			environment.Push((SCNumber)environment.Pop().As<SCArray>().Values.Count);
	}
}
