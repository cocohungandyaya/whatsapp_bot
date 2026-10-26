use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig};
use std::{thread::sleep,time::Duration};
use std::io::{self,Write};
use std::process::Command;
use std::fs;
//#main > div.x1n2onr6.x1vjfegm.x1cqoux5.x14yy4lh > div > div > div.x10l6tqk.x13vifvy.x1o0tod.xupqr0c.x9f619.x78zum5.xdt5ytf.xh8yej3.x5yr21d.x6ikm8r.x1rife3k.xjbqb8w.x1ewm37j > div.x3psx0u.x12xbjc7.x1c1uobl.x1yjeaew.xh8yej3.xquzyny.xvc5jky.x11t971q
//that is scan
//#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p
//this is input
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page("https://web.whatsapp.com/").await?;
    page.wait_for_navigation().await?;
    let mut temp:String=String::new();
    print!("Enter after done scan the QR code...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    let mut c_dialogue :String=String::new();
    let mut dialogue:String=String::new();
    match page.find_element("#main > div.x1n2onr6.x1vjfegm.x1cqoux5.x14yy4lh > div > div > div.x10l6tqk.x13vifvy.x1o0tod.xupqr0c.x9f619.x78zum5.xdt5ytf.xh8yej3.x5yr21d.x6ikm8r.x1rife3k.xjbqb8w.x1ewm37j > div.x3psx0u.x12xbjc7.x1c1uobl.x1yjeaew.xh8yej3.xquzyny.xvc5jky.x11t971q").await?.inner_text().await? {
        Some(i)=>{c_dialogue=i.to_string()},
        None=>{}
    }
    println!("{:?}",c_dialogue);
    loop{
        dialogue=c_dialogue.clone();
        match page.find_element("#main > div.x1n2onr6.x1vjfegm.x1cqoux5.x14yy4lh > div > div > div.x10l6tqk.x13vifvy.x1o0tod.xupqr0c.x9f619.x78zum5.xdt5ytf.xh8yej3.x5yr21d.x6ikm8r.x1rife3k.xjbqb8w.x1ewm37j > div.x3psx0u.x12xbjc7.x1c1uobl.x1yjeaew.xh8yej3.xquzyny.xvc5jky.x11t971q").await?.inner_text().await? {
            Some(i)=>{c_dialogue=i},
            None=>{}
        }
        println!("{:?}",c_dialogue);
        let cc_dialogue=c_dialogue.clone();
        let list_c_dialogue=cc_dialogue.split("\n").collect::<Vec<_>>();
        let ttemp=temp.clone();
        let list_temp=ttemp.split(" ").collect::<Vec<_>>();
        for x in list_temp{
                if !c_dialogue.contains(x){
                        temp=temp.split(x).collect::<Vec<_>>().join("");
                }
        }
        for i in list_c_dialogue{
            if (i.contains("/v ")||i.contains("/fv ")||i.contains(" /v")||i.contains(" /fv")||i.contains(" /sv")||i.contains("/sv ")||i.contains("/m ")||i.contains(" /m")) && !(dialogue.contains(i)) && !(temp.contains(i)){
                let mut resolution:String=String::new();
                let mut rev:String=String::new();
                if i.contains(" /fv")||i.contains("/fv "){
                    rev="Highest".to_string();
                }else if i.contains("/sv ")||i.contains(" /sv"){
                    resolution="bv*[height<=360]+ba/b[height<=360]".to_string();
                    rev="360p".to_string();
                }else if i.contains("/m ")||i.contains(" /m"){
                    resolution="ba".to_string();
                    rev="MP3".to_string();
                }else{
                    resolution="bv*[height<=720]+ba/b[height<=720]".to_string();
                    rev="720p".to_string();
                }
                for j in i.split(" ").collect::<Vec<_>>(){
                    if j.contains("https"){
                        let rt=j.split("https").collect::<Vec<_>>();
                        let mut j:String="https".to_string();
                        j.push_str(rt[1]);
                        let j:&str=j.as_str();
                        page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str(format!("REQUEST RECIEVED,Resolution SET:{}",rev)).await?.press_key("Enter").await?;
                        let mut output=Command::new("yt-dlp").output().unwrap();
                        if j.contains("b23.tv"){
                            if resolution=="bv*[height<=720]+ba/b[height<=720]"{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist","-f",resolution.as_str(),"--cookies","cookie.txt",j,"-o","output"]).output().unwrap();
                            }else if resolution=="bv*[height<=360]+ba/b[height<=360]"{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist","-f",resolution.as_str(),"--cookies","cookie.txt",j,"-o","output"]).output().unwrap();
                            }else if resolution=="ba"{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","-x","--audio-format","mp3","--no-playlist","--cookies","cookie.txt",j,"-o","output"]).output().unwrap();
                            }else{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist","--cookies","cookie.txt",j,"-o","output"]).output().unwrap();
                            }
                        }else{
                            if resolution=="bv*[height<=720]+ba/b[height<=720]" {
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist","-f",resolution.as_str(),j,"-o","output"]).output().unwrap();
                            }else if resolution=="bv*[height<=360]+ba/b[height<=360]"{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist","-f",resolution.as_str(),j,"-o","output"]).output().unwrap();
                            }else if resolution=="ba"{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","-x","--audio-format","mp3","--no-playlist",j,"-o","output"]).output().unwrap();
                            }else{
                                output=Command::new("yt-dlp").args(["--user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--add-header","User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36","--no-playlist",j,"-o","output"]).output().unwrap();
                            }
                        }
                        let mut stdout = String::new();
                        if output.status.success() {
                            stdout=String::from_utf8_lossy(&output.stdout).to_string();
                            println!("Command output:\n{}", stdout);
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Command failed:\n{}", stderr);
                            page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                            c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                            temp.push_str(format!(" {}",i).as_str());
                            break;
                        }
                        if !(resolution=="ba"){
                            if stdout.contains("output.webm"){
                                let output = Command::new("ffmpeg").args(["-i","output.webm","output.mp4"]).output().unwrap();
                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    println!("Command output:\n{}", stdout);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Command failed:\n{}", stderr);
                                    page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                                    c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                                    temp.push_str(format!(" {}",i).as_str());
                                    break;
                                }
                                fs::remove_file("output.webm")?;
                                println!("File deleted successfully");
                            }
                            else if stdout.contains("output.mkv"){
                                let output = Command::new("ffmpeg").args(["-i","output.mkv","output.mp4"]).output().unwrap();
                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    println!("Command output:\n{}", stdout);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Command failed:\n{}", stderr);
                                    page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                                    c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                                    temp.push_str(format!(" {}",i).as_str());
                                    break;
                                }
                                fs::remove_file("output.mkv")?;
                                println!("File deleted successfully");
                            }
                            else if stdout.contains("output.flv"){
                                let output = Command::new("ffmpeg").args(["-i","output.flv","output.mp4"]).output().unwrap();
                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    println!("Command output:\n{}", stdout);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Command failed:\n{}", stderr);
                                    page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                                    c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                                    temp.push_str(format!(" {}",i).as_str());
                                    break;
                                }
                                fs::remove_file("output.flv")?;
                                println!("File deleted successfully");
                            }
                            else if stdout.contains("output.3gp"){
                                let output = Command::new("ffmpeg").args(["-i","output.3gp","output.mp4"]).output().unwrap();
                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    println!("Command output:\n{}", stdout);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Command failed:\n{}", stderr);
                                    page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                                    c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                                    temp.push_str(format!(" {}",i).as_str());
                                    break;
                                }
                                fs::remove_file("output.3gp")?;
                                println!("File deleted successfully");
                            }
                        }
                        let mut output=Command::new("yt-dlp").output().unwrap();
                        if resolution=="ba"{
                            output = Command::new("powershell").args(["Set-Clipboard","-Path","output.mp3"]).output().unwrap();
                        }else{
                            output = Command::new("powershell").args(["Set-Clipboard","-Path","output.mp4"]).output().unwrap();
                        }
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            println!("Command output:\n{}", stdout);
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Command failed:\n{}", stderr);
                            page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                            c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                            temp.push_str(format!(" {}",i).as_str());
                            break;
                        }
                        page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?;
                        let output = Command::new("nircmd.exe").args(["sendkeypress","ctrl+v"]).output().unwrap();
                        let mut stdout = String::new();
                        if output.status.success() {
                            stdout=String::from_utf8_lossy(&output.stdout).to_string();
                            println!("Command output:\n{}", stdout);
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Command failed:\n{}", stderr);
                            page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.type_str("FAIL to download,please retry later").await?.press_key("Enter").await?;
                            c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                            temp.push_str(format!(" {}",i).as_str());
                            break;
                        }
                        if resolution=="ba"{
                            sleep(Duration::from_secs(30));
                        }else{
                            sleep(Duration::from_secs(60));
                        }
                        page.find_element("#main > footer > div.xnpuxes.x1n2onr6.xhtitgo.x9f619.x78zum5.x1q0g3np.xuk3077.xjbqb8w.x1wiwyrm.xquzyny.xvc5jky.x11t971q.copyable-area > div > span > div > div > div > div.x1n2onr6.xh8yej3.xjdcl3y > div.x1hx0egp.x6ikm8r.x1odjw0f.x1k6rcq7.x6prxxf > p").await?.press_key("Enter").await?;
                        if resolution=="ba"{
                            sleep(Duration::from_secs(30));
                            fs::remove_file("output.mp3");
                        }else{
                            fs::remove_file("output.mp4");
                        } 
                        println!("File deleted successfully");
                        c_dialogue=c_dialogue.split(i).collect::<Vec<_>>().join("");
                        temp.push_str(format!(" {}",i).as_str());
                        break;
                    }
                }
            }else{
                dialogue=dialogue.split(i).collect::<Vec<_>>().join("");
            }
        }
        sleep(Duration::from_secs(2));    
    }
    handle.await?;
    Ok(())
}
