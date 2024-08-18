mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use rust_xlsxwriter::*;
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn test_all() {
        let json = json!(
            [{"name":"ALKHALIDI SUHAIL SADAM MOHAMMED","category":"国际学生","from":"","sex":"男","relations":[{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["0086-18176400448"],"qq":[],"wechat":[]}}},{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":[],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242200058","drive_id":"","contact":{"phone":["0086-18176400448"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},{"name":"LEE TET YEE","category":"国际学生","from":"","sex":"男","relations":[{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["0060-189004913"],"qq":[],"wechat":[]}}},{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":[],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242200059","drive_id":"","contact":{"phone":["0060-127108178"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}}]
        );

        //println!("{:?}", json.is_array());

        let mut workbook = Workbook::new();

        let worksheet = workbook.add_worksheet();

        array(worksheet, json, "name").unwrap();

        workbook.save("test.xlsx").unwrap();
    }

    //#[test]
    fn test_homo() {
        let json = json!([{"name":"白千帆","category":"大陆生","from":"福建省","sex":"男","relations":[{"name":"白晓冬","from":"","sex":"男","relate_name":"父亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["18005906858"],"qq":[],"wechat":[]}}},{"name":"刘悦","from":"","sex":"女","relate_name":"母亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["17759230896"],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242200984","drive_id":"","contact":{"phone":["18060759106"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        {"name":"白统","category":"大陆生","from":"河南省","sex":"男","relations":[{"name":"王喜梅","from":"","sex":"女","relate_name":"母亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["18737702324"],"qq":[],"wechat":[]}}},{"name":"白春阳","from":"","sex":"男","relate_name":"父亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["18272770160"],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242200985","drive_id":"","contact":{"phone":["15203825067"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        {"name":"陈滢","category":"大陆生","from":"湖南省","sex":"女","relations":[{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["15673528479"],"qq":[],"wechat":[]}}},{"name":"","from":"","sex":"","relate_name":"","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":[],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242201010","drive_id":"","contact":{"phone":["15673528479"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        {"name":"朱敏浩","category":"大陆生","from":"安徽省","sex":"男","relations":[{"name":"朱立军","from":"","sex":"男","relate_name":"父亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13731841303"],"qq":[],"wechat":[]}}},{"name":"汪燕君","from":"","sex":"女","relate_name":"母亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13675594299"],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242201369","drive_id":"","contact":{"phone":["18815595596"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        {"name":"杜文婷","category":"大陆生","from":"海南省","sex":"女","relations":[{"name":"杜辉","from":"","sex":"男","relate_name":"父亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13976085050"],"qq":[],"wechat":[]}}},{"name":"魏香玲","from":"","sex":"女","relate_name":"母亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13976328008"],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242201024","drive_id":"","contact":{"phone":["15120992990"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        {"name":"艾力凯木·吾买尔","category":"大陆生","from":"新疆维吾尔自治区","sex":"男","relations":[{"name":"吾买尔·木合买提","from":"","sex":"男","relate_name":"父亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13364926063"],"qq":[],"wechat":[]}}},{"name":"热孜亚·尼亚孜","from":"","sex":"女","relate_name":"母亲","education":[],"work":[],"id":{"id":"","drive_id":"","contact":{"phone":["13239955851"],"qq":[],"wechat":[]}}}],"education":[],"work":[],"id":{"id":"","student_id":"34520242201373","drive_id":"","contact":{"phone":["19859286335","19859286335"],"qq":[],"wechat":[]},"class":{"class":"","class_qq":[],"grade":"","school":{"name":"","duration":{"start":"","end":""}}}}},
        ]);

        let arr = json.as_array().unwrap();
        let ha = HomoArr::new(arr);
        ha.run().unwrap();
    }
}
