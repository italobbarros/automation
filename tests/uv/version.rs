use automation::uv::commands as uv;

#[test]
fn test_version_more_equal_than() {
    // Versões acima da versão mínima
    assert!(uv::is_valid_python_version(">=3.10", "3.11.11"));
    assert!(uv::is_valid_python_version(">=3.10", "3.12.0"));
    assert!(uv::is_valid_python_version(">=3.10", "3.13.5"));

    // Versões iguais à versão mínima ou proximas
    assert!(uv::is_valid_python_version(">=3.10", "3.10.0"));
    assert!(uv::is_valid_python_version(">=3.10", "3.10.1"));

     // Versões abaixo da versão mínima
     assert!(!uv::is_valid_python_version(">=3.10", "3.9.9"));
     assert!(!uv::is_valid_python_version(">=3.10", "3.8.10"));
}

