use sp_std::prelude::*;
use frame_support::sp_runtime::{MultiSignature, AccountId32};
use frame_support::sp_runtime::app_crypto::{Public, sr25519, ed25519, Pair};
use frame_support::sp_runtime::traits::{Verify, IdentifyAccount};
use sp_core::hexdisplay::HexDisplay;

type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;

pub fn make_author_insert_key_params<'a>(raw_data: (&'a str, &'a str)) -> (&'a str, &'a str, String) {
    let account_id: [u8; 32];
    // println!("raw_data.0 = {:?}", raw_data.0);
    if "gran" == raw_data.0 {
        // let account_id = ed25519::Pair::from_string(raw_data.1, None)
        // .expect("Seed error of ed25519.").public().to_vec();
        account_id = extract_hex_of_public::<ed25519::Public>(raw_data.1);
    } else {
        account_id = extract_hex_of_public::<sr25519::Public>(raw_data.1);
    }
    //
    let account_id = HexDisplay::from(&account_id);
    // // println!("account_id = {:?}", &account_id);
    (raw_data.0, raw_data.1, format!("0x{}", &account_id))
}

pub fn extract_hex_of_public<TPublic: Public>(raw_data: &str) -> [u8; 32]
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
//  MultiSigner: From<<<TPublic as CryptoType>::Pair as sp_core::Pair>::Public>
{
    let account_id = TPublic::Pair::from_string(raw_data, None).expect("Seed error").public();
    let account_id: AccountId32 = AccountPublic::from(account_id).into_account();
    account_id.into()
}

pub fn extract_content(content: &str) -> Vec<(&str, &str)> {
    let content_split = content.split("\n");
    let line_list: Vec<&str> = content_split.collect();
    line_list.iter().map(|line| {
        let raw_split = line.split(":");
        let raw_list: Vec<&str> = raw_split.collect();
        assert_eq!(raw_list.len(), 2, "The format of the raw data line is incorrect.");
        (raw_list[0], raw_list[1])
    }).collect()
}

pub fn make_rpc_request(rpc_type: &str, param: (&str, &str, &str)) -> Option<String> {
    // {
    // 	"jsonrpc":"2.0",
    // 	"id":1,
    // 	"method":"author_insertKey",
    // 	"params": [
    // 	"aura",
    // 	"blur pioneer frown science banana impose avoid law act strategy have bronze//2//stash",
    // 	"0xc82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"
    // 	]
    // }

    if "author_insertKey" == rpc_type {
        let rcp_info = format!("{{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_insertKey\", \"params\":[\"{}\", \"{}\", \"{}\"]}}", param.0, param.1, param.2);
        return Some(rcp_info);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn test_account() {
        use super::*;

        let content = test_file_content();
        // println!("content: = {:?}", content);

        let rawkey_list = extract_content(content.as_str());
        // println!("rawkey_list = {:?} ", &rawkey_list);

        //
        let insert_key_list: Vec<(&str, &str, String)> = rawkey_list.iter().map(|x| {
            make_author_insert_key_params(*x)
        }).collect();

        let result_key_list = vec![
            ("ares", "blur pioneer frown science banana impose avoid law act strategy have bronze//1//stash",
             String::from("0x70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657")),
            ("gran", "blur pioneer frown science banana impose avoid law act strategy have bronze//1//grandpa",
             String::from("0x3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f")),
        ];
        assert_eq!(result_key_list, insert_key_list);

        let rpc_list: Vec<Option<String>> = result_key_list.iter().map(|x| {
            make_rpc_request("author_insertKey", (x.0, x.1, x.2.as_str()))
        }).collect();

        assert_eq!(2, rpc_list.len());

        // rpc_list.iter().any(|x|{
        // 	if let Some(rpc_body) = x {
        // 		println!("rpc_body = {:?}", rpc_body);
        // 	}
        // 	false
        // });
        // println!("rpc_list = {:?}", rpc_list);rpc_list.iter().any(|x|{
        // 	if let Some(rpc_body) = x {
        // 		println!("rpc_body = {:?}", rpc_body);
        // 	}
        // 	false
        // });
        // println!("rpc_list = {:?}", rpc_list);
    }

    // file extract
    pub fn test_file_content() -> String {
        let mut file = std::fs::File::open("/Users/mac/work-files/coding/git-files/ke-fan/ares-chain/seed-reader/src/test_authority_raw.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}



