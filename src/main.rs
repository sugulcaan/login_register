
fn main() {

    let mut database: Vec<Account> = Vec::new();


    register("Işıl".to_string(), "Işıl58".to_string(),
     "ışıl@58.com".to_string(), "Işıl123".to_string(),&mut database);

     login("ışıl@58.com".to_string(), "Işıl1234".to_string(), &mut database);
     let sayi=1;

}




#[derive(Debug,Clone)]
struct Account {

    name:String,
    username:String,
    email:String,
    password:String,
}


fn register(name:String,username:String,email:String,password:String, database:&mut Vec<Account>) {

    if !email.contains("@") {
        println!("Mail adresiniz hatalı")
        
    }
    else {
        let account = Account{
            name:name,
            username:username,
            email:email,
            password:password
        };

        database.push(account.clone());

        println!("Sistemimize kaydoldunuz {}",account.name)
    }


}

fn login(email:String,password:String,database:&mut Vec<Account>) {

    for data in database  {

        if email ==data.email && password == data.password {
            println!("Şifre ve mail doğru başarılı giriş yaptınız {}",data.username)            
        }
        else {
            println!("Şifre veya mail hatalı")
        }
        
    }
    
}
    




