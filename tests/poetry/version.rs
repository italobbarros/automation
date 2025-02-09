use automation::poetry::commands as poetry;


#[test]
fn test_version_into_range() {
    // Versões dentro do range
    assert!(poetry::is_valid_python_version("^3.11,<3.12", "3.11.11"));

    // Versões iguais à versão mínima do range
    assert!(poetry::is_valid_python_version("^3.11,<3.12", "3.11.0"));

    // Versões iguais à versão máxima do range
    assert!(!poetry::is_valid_python_version("^3.11,<3.12", "3.12.0"));

    // Versões fora do range
    assert!(!poetry::is_valid_python_version("^3.11,<3.12", "3.8.10"));
    assert!(!poetry::is_valid_python_version("^3.11,<3.12", "3.9.9"));
    assert!(!poetry::is_valid_python_version("^3.11,<3.12", "3.12.0"));
    assert!(!poetry::is_valid_python_version("^3.11,<3.12", "3.13.5"));
}
