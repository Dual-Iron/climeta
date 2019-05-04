use climeta::{Database, Cache};
use climeta::schema::{RetTypeKind, Type, TypeDef, TypeDefOrRef};

pub fn mmap_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<memmap::Mmap> {
    let file = std::fs::File::open(path.as_ref())?;
    unsafe { memmap::Mmap::map(&file) }
}

fn print_typedef(row: &TypeDef) -> Result<(), Box<std::error::Error>> {
    println!("{}.{} ({:?})", row.type_namespace()?, row.type_name()?, row.flags()?.semantics());

    for md in row.method_list()? {
        let sig = md.signature()?;

        println!(" - M {} with {} param(s)", md.name()?, sig.params().len());
        
        for (mpar, mpar_t) in md.param_list()?.skip_while(|p| if let Ok(0) = p.sequence() { true } else { false }).zip(sig.params()) {
            let flags = mpar.flags()?;
            let inout = match (flags.in_(), flags.out()) {
                (true, true) => "In/Out", // probably never happens
                (true, false) => "In",
                (false, true) => "Out",
                (false, false) => "-" // happens for constructors
            };
            println!("   - P {} {}: [{}] {:?}", mpar.sequence()?, mpar.name()?, inout, mpar_t.kind());
        }
        
        let ret = sig.return_type().kind();
        match ret {
            RetTypeKind::Type(Type::Ref(_, TypeDefOrRef::TypeRef(t), _)) => println!("   - R {}.{}", t.type_namespace()?, t.type_name()?),
            _ => println!("   - R {:?}", ret)
        }
    }
    // for fld in row.field_list()? {
    //     println!(" F {}", fld.name()?);
    // }

    match row.extends()? {
        None => println!(" Extends: <None>"),
        Some(TypeDefOrRef::TypeDef(def)) => {
            println!(" Extends: {}.{} (def)", def.type_namespace()?, def.type_name()?);
        },
        Some(TypeDefOrRef::TypeRef(def)) => {
            println!(" Extends: {}.{} (ref: {:?}) ", def.type_namespace()?, def.type_name()?, def.resolution_scope()?);
        },
        _ => ()
    }

    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    println!("=== Windows.Foundation.winmd ===");
    let cache = Cache::new();
    let db = cache.insert(Database::from_file("C:\\Windows\\System32\\WinMetadata\\Windows.Foundation.winmd")?);
    let typedefs = db.table::<TypeDef>();
    for row in typedefs {
        print_typedef(&row)?;
    }
    println!("TOTAL: {} == {}", typedefs.size(), typedefs.iter().count());
    println!("Looking for Windows.Foundation.Point ...");
    print_typedef(&cache.find("Windows.Foundation", "Point").unwrap())?;


    // println!("Typespecs:");
    // for ts in db.get_table::<TypeSpec>() {
    //     println!("- {:?}", ts.signature()?);
    // }

    // for cons in db.get_table::<schema::Constant>() {
    //     let parent = cons.parent()?;
    //     println!("{:?}, parent: {:?}", cons.type_()?, parent);
    //     if let Some(schema::HasConstant::Field(f)) = parent {
    //         println!("  {} -> {:?}", f.name()?, cons.value()?);
    //     }
    // }

    // for ms in db.get_table::<schema::MethodSemantics>() {
    //     let meth = ms.method()?;
    //     let sem = if ms.semantics()?.getter() {
    //         "getter"
    //     } else if ms.semantics()?.setter() {
    //         "setter"
    //     } else {
    //         "..."
    //     };
    //     println!("Semantics for method {:?}: {:?}", meth.name()?, sem);
    // }
    
    println!("=== Windows.UI.Xaml.winmd ===");
    let db2 = cache.insert(Database::from_file("C:\\Windows\\System32\\WinMetadata\\Windows.UI.Xaml.winmd")?);
    //let f2 = mmap_file("C:\\Windows\\System32\\WinMetadata\\Windows.UI.Xaml.winmd").unwrap();
    let typedefs = db2.table::<TypeDef>();
    for row in typedefs {
        //print_typedef(&row)?;
    }

    // for row in typedefs {
    //     println!("{}.{}", row.type_namespace()?, row.type_name()?);

        
    //     // for md in row.method_list()? {
    //     //     println!(" M {}", md.name()?);
    //     // }
    //     // for fld in row.field_list()? {
    //     //     println!(" F {}", fld.name()?);
    //     // }

    //     match row.extends()? {
    //         None => println!("  Extends: <None>"),
    //         Some(TypeDefOrRef::TypeDef(def)) => println!("  Extends: {}.{} (def)", def.type_namespace()?, def.type_name()?),
    //         Some(TypeDefOrRef::TypeRef(def)) => println!("  Extends: {}.{} (ref)", def.type_namespace()?, def.type_name()?),
    //         _ => ()
    //     }
    // }

    println!("TOTAL: {}", typedefs.size());
    println!("TOTAL (Foundation): {}", db.table::<TypeDef>().size());

    Ok(())
}