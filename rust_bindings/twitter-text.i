
%include <std_shared_ptr.i>
%shared_ptr(Configuration);

%module twitter_text
%{
/* Includes the header in the wrapper code */
#include "rust_bindings/cpp/twitter.h"
%}


%include <std_string.i>
%include <stdint.i>

// Some simple versions of the Rust headers
namespace rust {
	class String final {
	public:
	  String() noexcept;
	  String(const String &) noexcept;
	  String(String &&) noexcept;
	  ~String() noexcept;

	  String(const std::string &);
	  String(const char *);

	  String &operator=(const String &) noexcept;
	  String &operator=(String &&) noexcept;

	  explicit operator std::string() const;

	  // Note: no null terminator.
	  const char *data() const noexcept;
	  size_t size() const noexcept;
	  size_t length() const noexcept;

	private:
	  // Size and alignment statically verified by rust_string.rs.
	  std::array<uintptr_t, 3> repr;
	};
}

%rename("%(undercase)s", %$isfunction) "";

namespace rust {
	class String;
	%typemap(out) String {
	   $result = PyUnicode_FromStringAndSize($1.data(), $1.size());
	}

}

%ignore Extractor;
%ignore ExtractResult;
%ignore ValidatingExtractor;
%ignore HitHighlighter;
%ignore TwitterTextParser;
%ignore Validator;


/* Parse the header file to generate wrappers */
%include "rust_bindings/cpp/twitter.h"