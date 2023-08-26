pub trait Publish {
    fn publish_article(&self)->String;
} 

pub struct WeiBo {
    pub author: String,
    pub article_content:String,
}

impl Publish for WeiBo {
    fn publish_article(&self)->String {
        format!("content:{},author:{}",self.article_content,self.author)
    }
}