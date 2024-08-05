
// Contract Head
StructFieldStruct!{ ContractHead, 
	marks: Fixed10
}

// Contract System Call
StructFieldStruct!{ ContractSystemCall, 
    mark: Fixed2
	sign: Uint1
    code: BytesW2
}

// Contract User Func
StructFieldStruct!{ ContractClientFunc, 
    mark: Fixed6
	sign: Uint4
    code: BytesW2
}

// Func List
StructFieldList!(ContractSystemCallList, fnums, Uint1, funcs, ContractSystemCall);
StructFieldList!(ContractClientFuncList, fnums, Uint2, funcs, ContractClientFunc);


//////////////////////////////////////



// Contract
StructFieldStruct!{ Contract, 
	headmarks: ContractHead
	sytmcalls: ContractSystemCallList
	functions: ContractClientFuncList
}
