use std::time::Instant;
use ascent::ascent_par;
use ascent::rayon::ThreadPoolBuilder;
use clap::{arg, Command};
use zip::read::ZipFile;

mod utils;

// pub type Str = &'static str;

ascent_par! {
    relation isType(String);
    relation isReferenceType(String);
    relation isArrayType(String);
    relation isClassType(String);
    relation isInterfaceType(String);
    relation DirectSuperclass(String, String);
    relation DirectSuperinterface(String, String);
    relation ApplicationClass(String);
    relation MainClass(String);
    relation Field_DeclaringType(String, String);
    relation Method_DeclaringType(String, String);
    relation Method_ReturnType(String, String);
    relation Method_SimpleName(String, String);
    relation Method_Params(String, String);
    relation Method_Descriptor(String, String);
    relation Method_Modifier(String, String);

    relation FormalParam(String, String, String);
    relation ThisVar(String, String);
    relation Var_Type(String, String);
    relation Var_DeclaringMethod(String, String);
    relation HeapAllocation_Type(String, String);
    relation MainMethodArgArray(String);
    relation MainMethodArgArrayContent(String);
    relation Instruction_Method(String, String);

    relation isVirtualMethodInvocation_Insn(String);
    relation isStaticMethodInvocation_Insn(String);
    relation FieldInstruction_Signature(String, String);
    relation LoadInstanceField_Base(String, String);
    relation LoadInstanceField_To(String, String);
    relation StoreInstanceField_From(String, String);
    relation StoreInstanceField_Base(String, String);
    relation LoadStaticField_To(String, String);
    relation StoreStaticField_From(String, String);

    relation ComponentType(String, String);
    relation LoadArrayIndex_Base(String, String);
    relation LoadArrayIndex_To(String, String);
    relation StoreArrayIndex_From(String, String);
    relation StoreArrayIndex_Base(String, String);
    relation AssignInstruction_To(String, String);
    relation AssignCast_From(String, String);
    relation AssignCast_Type(String, String);

    relation AssignLocal_From(String, String);
    relation AssignHeapAllocation_Heap(String, String);
    relation ReturnNonvoid_Var(String, String);
    relation MethodInvocation_Method(String, String);
    relation AssignReturnValue(String, String);
    relation ActualParam(String, String, String);
    relation VirtualMethodInvocation_Base(String, String);
    relation VirtualMethodInvocation_SimpleName(String, String);
    relation VirtualMethodInvocation_Descriptor(String, String);
    relation SpecialMethodInvocation_Base(String, String);
    relation MethodInvocation_Base(String, String);
    // Fat schema
    relation LoadInstanceField(String, String, String, String);
    relation StoreInstanceField(String, String, String, String);
    relation LoadStaticField(String, String, String);
    relation StoreStaticField(String, String, String);
    relation LoadArrayIndex(String, String, String);
    relation StoreArrayIndex(String, String, String);
    relation AssignCast(String, String, String, String);
    relation AssignLocal(String, String, String);
    relation AssignHeapAllocation(String, String, String);
    relation ReturnVar(String, String);
    relation StaticMethodInvocation(String, String, String);

    relation _ClassType(String);
    isType(class) <-- _ClassType(class);
    isReferenceType(class) <-- _ClassType(class);
    isClassType(class) <-- _ClassType(class);
    relation _ArrayType(String);
    isType(arrayType) <-- _ArrayType(arrayType);
    isReferenceType(arrayType) <-- _ArrayType(arrayType);
    isArrayType(arrayType) <-- _ArrayType(arrayType);

    relation _InterfaceType(String);
    isType(interface) <--
      _InterfaceType(interface);
    isReferenceType(interface) <--
      _InterfaceType(interface);
    isInterfaceType(interface) <--
      _InterfaceType(interface);
    relation _Var_DeclaringMethod(String, String);
    Var_DeclaringMethod(var, method) <--
      _Var_DeclaringMethod(var, method);
    relation _ApplicationClass(String);
    isType(_type) <--
      _ApplicationClass(_type);
    isReferenceType(_type) <--
      _ApplicationClass(_type);
    ApplicationClass(_type) <--
      _ApplicationClass(_type);
    relation _ThisVar(String, String);
    relation _NormalHeap(String, String);
    relation _StringConstant(String);
    _ThisVar(method, var) <--
      _ThisVar(method, var);
    isType(_type),
    HeapAllocation_Type(id, _type) <--
      _NormalHeap(id, _type);
    HeapAllocation_Type(id, "java.lang.String".parse().unwrap()) <--
      _StringConstant(id);
    relation _AssignHeapAllocation(String, String, String, String, String, String);
    relation _AssignLocal(String, String, String, String, String);
    Instruction_Method(_instruction, _method),
    AssignInstruction_To(_instruction, _to),
    AssignHeapAllocation_Heap(_instruction, _heap) <--
      _AssignHeapAllocation(_instruction, _index, _heap, _to, _method, _linenumber);
    Instruction_Method(_instruction, _method),
    AssignLocal_From(_instruction, _from),
    AssignInstruction_To(_instruction, _to) <--
      _AssignLocal(_instruction, _index, _from, _to, _method);

    relation _AssignCast(String, String, String, String, String, String);
    relation _Field(String, String, String, String);
    Instruction_Method(instruction, method),
    AssignCast_Type(instruction, _type),
    AssignCast_From(instruction, _from),
    AssignInstruction_To(instruction, _to) <--
      _AssignCast(instruction, _index, _from, _to, _type, method);
    Field_DeclaringType(signature, declaringType) <--
      _Field(signature, declaringType, _, _);
    MethodInvocation_Base(invocation, base) <--
      VirtualMethodInvocation_Base(invocation, base);
    MethodInvocation_Base(invocation, base) <--
      SpecialMethodInvocation_Base(invocation, base);

    relation _StaticMethodInvocation(String, String, String, String);
    relation _SpecialMethodInvocation(String, String, String, String, String);
    relation _VirtualMethodInvocation(String, String, String, String, String);
    relation _Method(String, String, String, String, String, String, String);
    Instruction_Method(instruction, method),
    isStaticMethodInvocation_Insn(instruction),
    MethodInvocation_Method(instruction, signature) <--
      _StaticMethodInvocation(instruction, index, signature, method);
    Instruction_Method(instruction, method),
    SpecialMethodInvocation_Base(instruction, base),
    MethodInvocation_Method(instruction, signature) <--
      _SpecialMethodInvocation(instruction, index, signature, base, method);
    Instruction_Method(instruction, method),
    isVirtualMethodInvocation_Insn(instruction),
    VirtualMethodInvocation_Base(instruction, base),
    MethodInvocation_Method(instruction, signature) <--
      _VirtualMethodInvocation(instruction, index, signature, base, method);
    Method_SimpleName(method, simplename),
    Method_Params(method, params),
    Method_DeclaringType(method, declaringType),
    Method_ReturnType(method, returnType) <--
      _Method(method, simplename, params, declaringType, returnType, jvmDescriptor, arity);
    Method_Descriptor(method, descriptor) <--
      Method_ReturnType(method, returnType),
      Method_Params(method, params),
      let descriptor = format!("{}({})", returnType, params); // 'cat' is concatenation of strings

    relation _StoreInstanceField(String, String, String, String, String, String);
    relation _LoadInstanceField(String, String, String, String, String, String);
    relation _StoreStaticField(String, String, String, String, String);
    Instruction_Method(instruction, method),
    FieldInstruction_Signature(instruction, signature),
    StoreInstanceField_Base(instruction, base),
    StoreInstanceField_From(instruction, from) <--
        _StoreInstanceField(instruction, index, from, base, signature, method);
    Instruction_Method(instruction, method),
    FieldInstruction_Signature(instruction, signature),
    LoadInstanceField_Base(instruction, base),
    LoadInstanceField_To(instruction, to) <--
        _LoadInstanceField(instruction, index, to, base, signature, method);
    Instruction_Method(instruction, method),
    FieldInstruction_Signature(instruction, signature),
    StoreStaticField_From(instruction, from) <--
        _StoreStaticField(instruction, index, from, signature, method);

    relation _LoadStaticField(String, String, String, String, String);
    relation _StoreArrayIndex(String, String, String, String, String);
    relation _LoadArrayIndex(String, String, String, String, String);
    relation _Return(String, String, String, String);
    Instruction_Method(instruction, method),
    FieldInstruction_Signature(instruction, signature),
    LoadStaticField_To(instruction, to) <--
      _LoadStaticField(instruction, index, to, signature, method);
    Instruction_Method(instruction, method),
    StoreArrayIndex_Base(instruction, base),
    StoreArrayIndex_From(instruction, from) <--
      _StoreArrayIndex(instruction, index, from, base, method);
    Instruction_Method(instruction, method),
    LoadArrayIndex_Base(instruction, base),
    LoadArrayIndex_To(instruction, to) <--
      _LoadArrayIndex(instruction, index, to, base, method);
    Instruction_Method(instruction, method),
    ReturnNonvoid_Var(instruction, var) <--
      _Return(instruction, index, var, method);

    // fat schema population
    LoadInstanceField(base, sig, to, inmethod) <--
      Instruction_Method(insn, inmethod),
      LoadInstanceField_Base(insn, base),
      FieldInstruction_Signature(insn, sig),
      LoadInstanceField_To(insn, to);
    StoreInstanceField(from, base, sig, inmethod) <--
      Instruction_Method(insn, inmethod),
      StoreInstanceField_From(insn, from),
      StoreInstanceField_Base(insn, base),
      FieldInstruction_Signature(insn, sig);
    LoadStaticField(sig, to, inmethod) <--
      Instruction_Method(insn, inmethod),
      FieldInstruction_Signature(insn, sig),
      LoadStaticField_To(insn, to);
    StoreStaticField(from, sig, inmethod) <--
      Instruction_Method(insn, inmethod),
      StoreStaticField_From(insn, from),
      FieldInstruction_Signature(insn, sig);
    LoadArrayIndex(base, to, inmethod) <--
      Instruction_Method(insn, inmethod),
      LoadArrayIndex_Base(insn, base),
      LoadArrayIndex_To(insn, to);
    StoreArrayIndex(from, base, inmethod) <--
      Instruction_Method(insn, inmethod),
      StoreArrayIndex_From(insn, from),
      StoreArrayIndex_Base(insn, base);
    AssignCast(_type, from, to, inmethod) <--
      Instruction_Method(insn, inmethod),
      AssignCast_From(insn, from),
      AssignInstruction_To(insn, to),
      AssignCast_Type(insn, _type);
    AssignLocal(from, to, inmethod) <--
      AssignInstruction_To(insn, to),
      Instruction_Method(insn, inmethod),
      AssignLocal_From(insn, from);
    AssignHeapAllocation(heap, to, inmethod) <--
      Instruction_Method(insn, inmethod),
      AssignHeapAllocation_Heap(insn, heap),
      AssignInstruction_To(insn, to);
    ReturnVar(_var, _method) <--
      Instruction_Method(_insn, _method),
      ReturnNonvoid_Var(_insn, _var);

    StaticMethodInvocation(invocation, signature, inmethod) <--
      isStaticMethodInvocation_Insn(invocation),
      Instruction_Method(invocation, inmethod),
      MethodInvocation_Method(invocation, signature);
    HeapAllocation_Type(heap, _type),
    MainMethodArgArray(heap) <--
      let heap = "<<main method array>>",
      let _type = "java.lang.String[]";
    HeapAllocation_Type(heap, _type),
    MainMethodArgArrayContent(heap) <--
      let heap = "<<main method array content>>",
      let _type = "java.lang.String";
    VirtualMethodInvocation_SimpleName(invocation, simplename),
    VirtualMethodInvocation_Descriptor(invocation, descriptor) <--
      isVirtualMethodInvocation_Insn(invocation),
      MethodInvocation_Method(invocation, signature),
      Method_SimpleName(signature, simplename),
      Method_Descriptor(signature, descriptor);

    // Basic (type-based) analysis
    relation MethodLookup(String, String, String, String);
    relation MethodImplemented(String, String, String, String);
    relation DirectSubclass(String, String);
    relation Subclass(String, String);
    relation Superclass(String, String);
    relation Superinterface(String, String);
    relation SubtypeOf(String, String);
    relation SupertypeOf(String, String);
    relation SubtypeOfDifferent(String, String);
    relation MainMethodDeclaration(String);
    MethodLookup(simplename, descriptor, type_, method) <--
      MethodImplemented(simplename, descriptor, type_, method);
    MethodLookup(simplename, descriptor, type_, method) <--
      (DirectSuperclass(type_, supertype) || DirectSuperinterface(type_, supertype)),
      MethodLookup(simplename, descriptor, supertype, method),
      !MethodImplemented(simplename, descriptor, type_, _);
    MethodImplemented(simplename, descriptor, type_, method) <--
      Method_SimpleName(method, simplename),
      Method_Descriptor(method, descriptor),
      Method_DeclaringType(method, type_),
      !Method_Modifier("abstract".to_string(), method);
    DirectSubclass(a, c) <--
      DirectSuperclass(a, c);
    Subclass(c, a) <--
      DirectSubclass(a, c);
    Subclass(c, a) <--
      Subclass(b, a),
      DirectSubclass(b, c);
    Superclass(c, a) <--
      Subclass(a, c);
    Superinterface(k, c) <--
      DirectSuperinterface(c, k);
    Superinterface(k, c) <--
      DirectSuperinterface(c, j),
      Superinterface(k, j);
    Superinterface(k, c) <--
      DirectSuperclass(c, _super),
      Superinterface(k, _super);

    SupertypeOf(s, t) <--
      SubtypeOf(t, s);
    SubtypeOf(s, s) <--
      isClassType(s);
    SubtypeOf(s, t) <--
      Subclass(t, s);
    SubtypeOf(s, t) <--
      isClassType(s),
      Superinterface(t, s);
    SubtypeOf(s, t) <--
      isInterfaceType(s),
      isType(t),
      if t == "java.lang.Object";
    SubtypeOf(s, s) <--
      isInterfaceType(s);
    SubtypeOf(s, t) <--
      isInterfaceType(s),
      Superinterface(t, s);
    SubtypeOf(s, t) <--
      isArrayType(s),
      isType(t),
      if t == "java.lang.Object";
    SubtypeOf(s, t) <--
      ComponentType(s, sc),
      ComponentType(t, tc),
      isReferenceType(sc),
      isReferenceType(tc),
      SubtypeOf(sc, tc);
    //  不知道为什么 该规则会引入极长的耗时

    SubtypeOf(s, t) <--
      isArrayType(s),
      isInterfaceType(t),
      isType(t),
      if t == "java.lang.Cloneable";
    SubtypeOf(s, t) <--
      isArrayType(s),
      isInterfaceType(t),
      isType(t),
      if t == "java.io.Serializable";
    SubtypeOf(t, t) <--
      isType(t);
    SubtypeOfDifferent(s, t) <--
      SubtypeOf(s, t),
      if s != t;

    MainMethodDeclaration(method) <--
      MainClass(_type),
      Method_DeclaringType(method, _type),
      if method != "<java.util.prefs.Base64: void main(java.lang.String[])>",
      if method != "<sun.java2d.loops.GraphicsPrimitiveMgr: void main(java.lang.String[])>",
      if method != "<sun.security.provider.PolicyParser: void main(java.lang.String[])>",
      Method_SimpleName(method, "main".to_string()),
      Method_Descriptor(method, "void(java.lang.String[])".to_string()),
      Method_Modifier("public".to_string(), method),
      Method_Modifier("static".to_string(), method);

    // class initialization
    relation ClassInitializer(String, String);
    relation InitializedClass(String);
    ClassInitializer(_type, method) <--
      MethodImplemented("<clinit>".to_string(), "void()".to_string(), _type, method);
    InitializedClass(superclass) <--
      InitializedClass(_class),
      DirectSuperclass(_class, superclass);
    InitializedClass(superinterface) <--
      InitializedClass(classOrInterface),
      DirectSuperinterface(classOrInterface, superinterface);
    InitializedClass(_class) <--
      MainMethodDeclaration(method),
      Method_DeclaringType(method, _class);
    InitializedClass(_class) <--
      Reachable(inmethod),
      AssignHeapAllocation(heap, _, inmethod),
      HeapAllocation_Type(heap, _class);

    InitializedClass(class) <--
      Reachable(inmethod),
      Instruction_Method(invocation, inmethod),
      isStaticMethodInvocation_Insn(invocation),
      MethodInvocation_Method(invocation, signature),
      Method_DeclaringType(signature, class);
    InitializedClass(classOrInterface) <--
      Reachable(inmethod),
      StoreStaticField(_, signature, inmethod),
      Field_DeclaringType(signature, classOrInterface);
    InitializedClass(classOrInterface) <--
      Reachable(inmethod),
      LoadStaticField(signature, _, inmethod),
      Field_DeclaringType(signature, classOrInterface);
    Reachable(clinit) <--
      InitializedClass(class),
      ClassInitializer(class, clinit);

    relation Assign(String, String);
    relation VarPointsTo(String, String);
    relation InstanceFieldPointsTo(String, String, String);
    relation StaticFieldPointsTo(String, String);
    relation CallGraphEdge(String, String);
    relation ArrayIndexPointsTo(String, String);
    relation Reachable(String);

    Assign(actual, formal) <--
      CallGraphEdge(invocation, method),
      FormalParam(index, method, formal),
      ActualParam(index, invocation, actual);
    Assign(_return, local) <--
      CallGraphEdge(invocation, method),
      ReturnVar(_return, method),
      AssignReturnValue(invocation, local);
    VarPointsTo(heap, var) <--
      AssignHeapAllocation(heap, var, inMethod),
      Reachable(inMethod);
    VarPointsTo(heap, to) <--
      Assign(from, to),
      VarPointsTo(heap, from);
    VarPointsTo(heap, to) <--
      Reachable(inmethod),
      AssignLocal(from, to, inmethod),
      VarPointsTo(heap, from);

    VarPointsTo(heap, to) <--
      Reachable(method),
      AssignCast(_type, from, to, method),
      SupertypeOf(_type, heaptype),
      HeapAllocation_Type(heap, heaptype),
      VarPointsTo(heap, from);
    ArrayIndexPointsTo(baseheap, heap) <--
      Reachable(inmethod),
      StoreArrayIndex(from, base, inmethod),
      VarPointsTo(baseheap, base),
      VarPointsTo(heap, from),
      HeapAllocation_Type(heap, heaptype),
      HeapAllocation_Type(baseheap, baseheaptype),
      ComponentType(baseheaptype, componenttype),
      SupertypeOf(componenttype, heaptype);
    VarPointsTo(heap, to) <--
      Reachable(inmethod),
      LoadArrayIndex(base, to, inmethod),
      VarPointsTo(baseheap, base),
      ArrayIndexPointsTo(baseheap, heap),
      Var_Type(to, _type),
      HeapAllocation_Type(baseheap, baseheaptype),
      ComponentType(baseheaptype, basecomponenttype),
      SupertypeOf(_type, basecomponenttype);

    VarPointsTo(heap, to) <--
      Reachable(inmethod),
      LoadInstanceField(base, signature, to, inmethod),
      VarPointsTo(baseheap, base),
      InstanceFieldPointsTo(heap, signature, baseheap);
    InstanceFieldPointsTo(heap, fld, baseheap) <--
      Reachable(inmethod),
      StoreInstanceField(from, base, fld, inmethod),
      VarPointsTo(heap, from),
      VarPointsTo(baseheap, base);
    VarPointsTo(heap, to) <--
      Reachable(inmethod),
      LoadStaticField(fld, to, inmethod),
      StaticFieldPointsTo(heap, fld);
    StaticFieldPointsTo(heap, fld) <--
      Reachable(inmethod),
      StoreStaticField(from, fld, inmethod),
      VarPointsTo(heap, from);

    VarPointsTo(heap, this) <--
      Reachable(inMethod),
      Instruction_Method(invocation, inMethod),
      VirtualMethodInvocation_Base(invocation, base),
      VarPointsTo(heap, base),
      HeapAllocation_Type(heap, heaptype),
      VirtualMethodInvocation_SimpleName(invocation, simplename),
      VirtualMethodInvocation_Descriptor(invocation, descriptor),
      MethodLookup(simplename, descriptor, heaptype, toMethod),
      ThisVar(toMethod, this);
    Reachable(toMethod),
    CallGraphEdge(invocation, toMethod) <--
      Reachable(inMethod),
      Instruction_Method(invocation, inMethod),
      VirtualMethodInvocation_Base(invocation, base),
      VarPointsTo(heap, base),
      HeapAllocation_Type(heap, heaptype),
      VirtualMethodInvocation_SimpleName(invocation, simplename),
      VirtualMethodInvocation_Descriptor(invocation, descriptor),
      MethodLookup(simplename, descriptor, heaptype, toMethod);

    Reachable(toMethod),
    CallGraphEdge(invocation, toMethod) <--
      Reachable(inMethod),
      StaticMethodInvocation(invocation, toMethod, inMethod);
    Reachable(toMethod),
    CallGraphEdge(invocation, toMethod),
    VarPointsTo(heap, this) <--
      Reachable(inMethod),
      Instruction_Method(invocation, inMethod),
      SpecialMethodInvocation_Base(invocation, base),
      VarPointsTo(heap, base),
      MethodInvocation_Method(invocation, toMethod),
      ThisVar(toMethod, this);
    Reachable(method) <--
      MainMethodDeclaration(method);

}


fn input_process(prog: &mut AscentProgram, zip_file_name: &String) {

    // .input _Return(IO="file", filename="Return.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Return.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._Return.extend(tmp);
    // .input _LoadArrayIndex(IO="file", filename="LoadArrayIndex.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/LoadArrayIndex.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._LoadArrayIndex.extend(tmp);
    // .input _StoreArrayIndex(IO="file", filename="StoreArrayIndex.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/StoreArrayIndex.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._StoreArrayIndex.extend(tmp);
    // .input _LoadStaticField(IO="file", filename="LoadStaticField.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/LoadStaticField.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._LoadStaticField.extend(tmp);
    // .input _StoreStaticField(IO="file", filename="StoreStaticField.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/StoreStaticField.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._StoreStaticField.extend(tmp);
    // .input _LoadInstanceField(IO="file", filename="LoadInstanceField.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/LoadInstanceField.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._LoadInstanceField.extend(tmp);
    // .input _StoreInstanceField(IO="file", filename="StoreInstanceField.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/StoreInstanceField.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._StoreInstanceField.extend(tmp);
    // .input _Method(IO="file", filename="Method.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Method.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._Method.extend(tmp);
    // .input _VirtualMethodInvocation(IO="file", filename="VirtualMethodInvocation.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/VirtualMethodInvocation.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._VirtualMethodInvocation.extend(tmp);
    // .input _SpecialMethodInvocation(IO="file", filename="SpecialMethodInvocation.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/SpecialMethodInvocation.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._SpecialMethodInvocation.extend(tmp);
    // .input _StaticMethodInvocation(IO="file", filename="StaticMethodInvocation.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/StaticMethodInvocation.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._StaticMethodInvocation.extend(tmp);
    // .input _Field(IO="file", filename="Field.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Field.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._Field.extend(tmp);
    // .input _AssignCast(IO="file", filename="AssignCast.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/AssignCast.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._AssignCast.extend(tmp);
    // .input _AssignLocal(IO="file", filename="AssignLocal.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/AssignLocal.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._AssignLocal.extend(tmp);
    // .input _AssignHeapAllocation(IO="file", filename="AssignHeapAllocation.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/AssignHeapAllocation.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._AssignHeapAllocation.extend(tmp);
    // .input _StringConstant(IO="file", filename="StringConstant.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/StringConstant.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._StringConstant.extend(tmp);
    // .input _NormalHeap(IO="file", filename="NormalHeap.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/NormalHeap.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._NormalHeap.extend(tmp);
    // .input _ThisVar(IO="file", filename="ThisVar.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ThisVar.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._ThisVar.extend(tmp);
    //.input DirectSuperclass(IO="file", filename="DirectSuperclass.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/DirectSuperclass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperclass.extend(tmp);
    //.input DirectSuperinterface(IO="file", filename="DirectSuperinterface.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/DirectSuperinterface.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperinterface.extend(tmp);
    // .input MainClass(IO="file", filename="MainClass.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/MainClass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog.MainClass.extend(tmp);
    // .input Method_Modifier(IO="file", filename="Method-Modifier.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Method-Modifier.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.Method_Modifier.extend(tmp);
    // .input FormalParam(IO="file", filename="FormalParam.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/FormalParam.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.FormalParam.extend(tmp);
    // .input Var_Type(IO="file", filename="Var-Type.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Var-Type.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.Var_Type.extend(tmp);
    // .input ComponentType(IO="file", filename="ComponentType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ComponentType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.ComponentType.extend(tmp);
    // .input AssignReturnValue(IO="file", filename="AssignReturnValue.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/AssignReturnValue.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.AssignReturnValue.extend(tmp);
    // .input ActualParam(IO="file", filename="ActualParam.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ActualParam.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.ActualParam.extend(tmp);
    // .input _ClassType(IO="file", filename="ClassType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ClassType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._ClassType.extend(tmp);
    // .input _ArrayType(IO="file", filename="ArrayType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ArrayType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._ArrayType.extend(tmp);
    // .input _InterfaceType(IO="file", filename="InterfaceType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/InterfaceType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._InterfaceType.extend(tmp);
    // .input _Var_DeclaringMethod(IO="file", filename="Var-DeclaringMethod.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Var-DeclaringMethod.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog._Var_DeclaringMethod.extend(tmp);
    // .input _ApplicationClass(IO="file", filename="ApplicationClass.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ApplicationClass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._ApplicationClass.extend(tmp);
}

fn output_process(prog: &mut AscentProgram) {
    // relation Assign(String, String);
    println!("[OutPut]: Assign size: {:?}", prog.Assign.len());
    // relation VarPointsTo(String, String);
    println!("[OutPut]: VarPointsTo size: {:?}", prog.VarPointsTo.len());
    // relation InstanceFieldPointsTo(String, String, String);
    println!("[OutPut]: InstanceFieldPointsTo size: {:?}", prog.InstanceFieldPointsTo.len());
    // relation StaticFieldPointsTo(String, String);
    println!("[OutPut]: StaticFieldPointsTo size: {:?}", prog.StaticFieldPointsTo.len());
    // relation CallGraphEdge(String, String);
    println!("[OutPut]: CallGraphEdge size: {:?}", prog.CallGraphEdge.len());
    // relation ArrayIndexPointsTo(String, String);
    println!("[OutPut]: ArrayIndexPointsTo size: {:?}", prog.ArrayIndexPointsTo.len());
    // relation Reachable(String);
    println!("[OutPut]: Reachable size: {:?}", prog.Reachable.len());
}

fn main() {
    let matches = Command::new("Doop-thin Program")
        .version("1.0")
        .author("Tianqi Shen")
        .arg(arg!(--database <VALUE>).required(true))
        .get_matches();
    let zip_file_name = matches.get_one::<String>("database").expect("required");
    println!("zip file name: {}", zip_file_name);
    // utils::utils::list_zip_contents(zip_file_name)?;

    let _pool = ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();
    println!("Hello, world!");
    let mut prog = AscentProgram::default();
    // input & facts area below
    input_process(&mut prog, zip_file_name);

    // println!("[debug]: input process done");

    let start = Instant::now();
    prog.run();
    let duration = start.elapsed();
    println!("[OutPut]: 方法运行时间: {:?}", duration);
    // println!("{}", prog.Method_Descriptor.len());
    output_process(&mut prog);
    // println!("[NOTE]: 目前还有注释内容需要处理！")
    // let mut count = 0;  // 定义一个计数器
    // for (i, j) in prog.Method_Descriptor {
    //     if count % 1000 == 0 {  // 检查计数器是否是1000的倍数
    //         println!("{}，间隔， {}", i, j);
    //     }
    //     count += 1;  // 每次循环，计数器增加1
    // }
}
