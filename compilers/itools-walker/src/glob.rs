/// Glob 模式
#[derive(Debug, Clone)]
pub struct Glob {
    pattern: String,
}

impl Glob {
    /// 创建新的 Glob 模式
    pub fn new(pattern: &str) -> Result<Self, String> {
        Ok(Self { pattern: pattern.to_string() })
    }

    /// 匹配路径
    pub fn is_match(&self, path: &std::path::Path) -> bool {
        let path_str = path.to_string_lossy();
        self.match_pattern(&self.pattern, &path_str)
    }

    /// 匹配模式的内部实现
    fn match_pattern(&self, pattern: &str, path: &str) -> bool {
        let mut pattern_iter = pattern.chars();
        let mut path_iter = path.chars();

        loop {
            match (pattern_iter.next(), path_iter.next()) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some('*'), None) => return true,
                (Some('*'), Some(c)) => {
                    // 尝试匹配任意长度的字符
                    let remaining_pattern: String = pattern_iter.collect();
                    let remaining_path: String = path_iter.collect();

                    // 递归尝试匹配
                    return self.match_pattern(&remaining_pattern, &remaining_path)
                        || self.match_pattern(pattern, &format!("{}{}", c, remaining_path));
                }
                (Some('?'), Some(_)) => continue,
                (Some(pc1), Some(pc2)) if pc1 == pc2 => continue,
                _ => return false,
            }
        }
    }
}

/// Glob 模式集合
#[derive(Debug, Clone)]
pub struct GlobSet {
    globs: Vec<Glob>,
}

impl GlobSet {
    /// 检查路径是否匹配任何模式
    pub fn is_match(&self, path: &std::path::Path) -> bool {
        self.globs.iter().any(|glob| glob.is_match(path))
    }
}

/// Glob 模式集合构建器
pub struct GlobSetBuilder {
    globs: Vec<Glob>,
}

impl GlobSetBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { globs: Vec::new() }
    }

    /// 添加模式
    pub fn add(&mut self, glob: Result<Glob, String>) -> &mut Self {
        if let Ok(glob) = glob {
            self.globs.push(glob);
        }
        self
    }

    /// 构建 GlobSet
    pub fn build(&self) -> Result<GlobSet, String> {
        Ok(GlobSet { globs: self.globs.clone() })
    }
}

/// 构建 glob 模式集合
pub fn build_glob_set(include_patterns: &[String], exclude_patterns: &[String]) -> Result<GlobSet, crate::errors::WalkError> {
    let mut builder = GlobSetBuilder::new();

    // 添加排除模式
    for pattern in exclude_patterns {
        builder.add(Glob::new(pattern));
    }

    // 添加包含模式
    for pattern in include_patterns {
        builder.add(Glob::new(pattern));
    }

    match builder.build() {
        Ok(glob_set) => Ok(glob_set),
        Err(err) => Err(crate::errors::WalkError::GlobParse(err)),
    }
}

/// 检查路径是否应该被包含
pub fn should_include(path: &std::path::Path, glob_set: &Option<GlobSet>) -> bool {
    if let Some(glob_set) = glob_set {
        // 路径不能匹配任何排除模式
        !glob_set.is_match(path)
    }
    else {
        true
    }
}
