


# this is a test



## works

{% match x %}
{% when X::B with (i) %}
i = {{ i }}
{% when X::C %}
neither X/C
{% else %}
it's X/A
{% endmatch %}

## does not work


{% match x %}
{% when X::A with ( name : name ) %}
// with ( name ) nor does { name } nor does { name : name } nor does ( name : name ) work
name = {{ name }}
{% else %}
not X::A
{% endmatch %}
