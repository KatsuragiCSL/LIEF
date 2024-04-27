mod utils;
use lief::pe::debug::Entries as DebugEntries;
use lief::pe::load_configuration::{
    AsBase,
    AsLoadConfigV0, AsLoadConfigV1, AsLoadConfigV2,
    AsLoadConfigV3, AsLoadConfigV4, AsLoadConfigV5,
    AsLoadConfigV6, AsLoadConfigV7, AsLoadConfigV8,
    AsLoadConfigV9, AsLoadConfigV10,
    LoadConfiguration
};
use lief::pe::resources::{Node, NodeBase};
use lief::pe::Algorithms;
use lief::pe::signature::attributes::Attribute;
use lief::pe::signature::Signature;

use lief::generic::Binary as GenericBinary;
use lief::Binary;

fn get_leaf<'a>(node: &'a Node) -> Option<&'a Node<'a>> {
    match node {
        Node::Data(_) => {
            return Some(node);
        }

        Node::Directory(dir) => {
            for child in dir.children() {
                if let Some(Node::Data(data)) = get_leaf(&child) {
                    format!("{}", data.content().len());
                    format!("{data:?}");
                }
            }
        }
    }
    None
}

fn explore_sig_attribute(attr: &Attribute) {
    match attr {
        Attribute::ContentType(item) => {
            format!("{item:?}");
        }
        Attribute::GenericType(item) => {
            format!("{item:?}");
        }
        Attribute::MsSpcNestedSignature(item) => {
            format!("{item:?}");
            explore_signature(&item.signature());
        }
        Attribute::MsSpcStatementType(item) => {
            format!("{item:?}");
        }
        Attribute::PKCS9AtSequenceNumber(item) => {
            format!("{item:?}");
        }
        Attribute::PKCS9CounterSignature(item) => {
            format!("{item:?}");
            let signer = item.signer();
            for attr in signer.authenticated_attributes() {
                explore_sig_attribute(&attr);
            }

            for attr in signer.unauthenticated_attributes() {
                explore_sig_attribute(&attr);
            }
        }
        Attribute::PKCS9MessageDigest(item) => {
            format!("{item:?}");
            format!("{}", item.digest().len());
        }
        Attribute::PKCS9SigningTime(item) => {
            format!("{item:?}");
        }
        Attribute::SpcSpOpusInfo(item) => {
            format!("{item:?}");
        }
    }
}

fn explore_signature(signature: &Signature) {
    format!("{signature:?}");
    let content_info = signature.content_info();
    format!("{content_info:?}");
    for crt in signature.certificates() {
        format!("{crt:?}");
        format!("{}", crt.serial_number().len());
        format!("{}", crt.raw().len());
        format!("{}", crt.signature().len());

        if let Some(rsa_info) = crt.rsa_info() {
            format!("{rsa_info:?}");
            format!("{}", rsa_info.N().bits());
            format!("{}", rsa_info.E().bits());
            format!("{}", rsa_info.D().bits());
            format!("{}", rsa_info.P().bits());
            format!("{}", rsa_info.Q().bits());
        }
    }
    for signer in signature.signers() {
        format!("{signer:?}");
        if let Some(crt) = signer.cert() {
            format!("{crt:?}");
        }
        format!("{}", signer.encrypted_digest().len());
        format!("{}", signer.serial_number().len());

        for attr in signer.authenticated_attributes() {
            explore_sig_attribute(&attr);
        }

        for attr in signer.unauthenticated_attributes() {
            explore_sig_attribute(&attr);
        }
    }
}

fn explore_pe(_: &str, pe: &lief::pe::Binary) {
    format!("{pe:?}");

    let dos_header = pe.dos_header();
    format!("{dos_header:?}");

    let optional_header = pe.optional_header();
    format!("{optional_header:?}");

    let header = pe.header();
    format!("{header:?}");

    if let Some(tls) = pe.tls() {
        format!("{tls:?}");
        format!("{}", tls.data_template().len());
        format!("{}", tls.addressof_raw_data().len());
    }

    if let Some(export) = pe.export() {
        format!("{export:?}");
        for entry in export.entries() {
            format!("{entry:?}");
        }
    }

    if let Some(resources) = pe.resources() {
        format!("{resources:?}");
        match resources {
            Node::Data(_) => {}

            Node::Directory(dir) => {
                for child in dir.children() {
                    format!("{}", dir.depth());
                    format!("{child:?}");
                }
            }
        }
    }

    if let Some(resources) = pe.resources() {
        if let Some(leaf) = get_leaf(&resources) {
            format!("{leaf:?}");
        }
    }

    if let Some(config) = pe.load_configuration() {
        format!("{config:?}");
        match config {
            LoadConfiguration::Base(item) => {
                format!("{:?}", item);
            }
            LoadConfiguration::V0(item) => {
                let base = &item as &dyn AsBase;
                format!("{:?}", item);
                format!("{:?}", base);
            }
            LoadConfiguration::V1(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                format!("{:?}", item);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V2(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                format!("{:?}", item);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V3(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                format!("{:?}", item);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V4(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                format!("{:?}", item);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V5(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                format!("{:?}", item);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V6(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                format!("{:?}", item);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V7(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                let v6 = &item as &dyn AsLoadConfigV6;
                format!("{:?}", item);
                format!("{:?}", v6);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V8(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                let v6 = &item as &dyn AsLoadConfigV6;
                let v7 = &item as &dyn AsLoadConfigV7;
                format!("{:?}", item);
                format!("{:?}", v7);
                format!("{:?}", v6);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V9(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                let v6 = &item as &dyn AsLoadConfigV6;
                let v7 = &item as &dyn AsLoadConfigV7;
                let v8 = &item as &dyn AsLoadConfigV8;
                format!("{:?}", item);
                format!("{:?}", v8);
                format!("{:?}", v7);
                format!("{:?}", v6);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V10(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                let v6 = &item as &dyn AsLoadConfigV6;
                let v7 = &item as &dyn AsLoadConfigV7;
                let v8 = &item as &dyn AsLoadConfigV8;
                let v9 = &item as &dyn AsLoadConfigV9;
                format!("{:?}", item);
                format!("{:?}", v9);
                format!("{:?}", v8);
                format!("{:?}", v7);
                format!("{:?}", v6);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
            LoadConfiguration::V11(item) => {
                let base = &item as &dyn AsBase;
                let v0 = &item as &dyn AsLoadConfigV0;
                let v1 = &item as &dyn AsLoadConfigV1;
                let v2 = &item as &dyn AsLoadConfigV2;
                let v3 = &item as &dyn AsLoadConfigV3;
                let v4 = &item as &dyn AsLoadConfigV4;
                let v5 = &item as &dyn AsLoadConfigV5;
                let v6 = &item as &dyn AsLoadConfigV6;
                let v7 = &item as &dyn AsLoadConfigV7;
                let v8 = &item as &dyn AsLoadConfigV8;
                let v9 = &item as &dyn AsLoadConfigV9;
                let v10 = &item as &dyn AsLoadConfigV10;
                format!("{:?}", item);
                format!("{:?}", v10);
                format!("{:?}", v9);
                format!("{:?}", v8);
                format!("{:?}", v7);
                format!("{:?}", v6);
                format!("{:?}", v5);
                format!("{:?}", v4);
                format!("{:?}", v3);
                format!("{:?}", v2);
                format!("{:?}", v1);
                format!("{:?}", v0);
                format!("{:?}", base);
            }
        }
    }

    if let Some(header) = pe.rich_header() {
        format!("{header:?}");
        for entry in header.entries() {
            format!("{entry:?}");
        }
    }

    let _ = pe.resources_manager();

    for imp in pe.imports() {
        format!("{imp:?}");
        for entry in imp.entries() {
            format!("{entry:?}");
        }
    }

    for dir in pe.data_directories() {
        format!("{dir:?}");
    }

    for section in pe.sections() {
        format!("{section:?}");
        format!("{}", section.padding().len());
    }

    for relocation in pe.relocations() {
        format!("{relocation:?}");
        for entry in relocation.entries() {
            format!("{entry:?}");
        }
    }

    for delay_import in pe.delay_imports() {
        format!("{delay_import:?}");
        for entry in delay_import.entries() {
            format!("{entry:?}");
        }
    }

    for signature in pe.signatures() {
        explore_signature(&signature);
    }

    for debug in pe.debug() {
        format!("{debug:?}");
        match debug {
            DebugEntries::Pogo(pogo) => {
                for entry in pogo.entries() {
                    format!("{entry:?}");
                }
            }
            DebugEntries::Repro(repro) => {
                assert!(!repro.hash().is_empty());
            }
            _ => {}
        }
    }

    assert!(pe.entrypoint() > 0);

    assert!(!pe.dos_stub().is_empty());
    assert!(!pe.authentihash(Algorithms::SHA_256).is_empty());
}

fn test_with(bin_name: &str) {
    let path = utils::get_pe_sample(bin_name).unwrap();
    let path_str = path.to_str();
    if let Some(Binary::PE(bin)) = Binary::parse(path_str.unwrap()) {
        explore_pe(bin_name, &bin);
    }

    // Test Read + Seek interface
    let mut file = std::fs::File::open(path).expect("Can't open the file");
    let binary = Binary::from(&mut file);
    assert!(matches!(binary, Some(Binary::PE(_))));
}

fn test_signature(name: &str) {
    let path = utils::get_pkcs7_sample(name).unwrap();
    let path_str = path.to_str();
    if let Some(sig) = Signature::from_file(path_str.unwrap()) {
        explore_signature(&sig);
    }
}

#[test]
fn test_api() {
    test_with("ntoskrnl.exe");
    test_with("steam.exe");
    test_with("test.debug.repro.exe");
    test_with("PE64_x86-64_library_libLIEF.dll");
    test_with("PE32_x86-64_binary_avast-free-antivirus-setup-online.exe");
    test_with("PE32_x86-64_binary_self-signed.exe");
    test_with("steam.exe");

    test_signature("cert0.p7b");
    test_signature("cert3.p7b");
    test_signature("cert11.p7b");
    test_signature("cert9.p7b");
    test_signature("cert10.p7b");
}
