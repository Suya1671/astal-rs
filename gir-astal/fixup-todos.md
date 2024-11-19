# TODOs for fixup script

- [x] Constant updates

```xml
 <constant name="MAJOR_VERSION" c:identifier="ASTAL_MAJOR_VERSION" value="4">
  <type name="gint" c:type="gint" />
 </constant>
```

to

```xml
<constant name="MAJOR_VERSION" c:identifier="ASTAL_MAJOR_VERSION" value="4" type="ASTAL_MAJOR_VERSION">
    <type name="gint" c:type="gint" />
</constant>
```

- [x] remove *_VERSION since it is not used to my knowledge and breaks generation

- [x] Include AstalIO in Astal-4.0.gir
- [x] add shared-library declarations to namespaces

- [ ] fixup package names to include the version number