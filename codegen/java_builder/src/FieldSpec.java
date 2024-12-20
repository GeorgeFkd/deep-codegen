//https://github.com/palantir/javapoet/blob/develop/javapoet/src/main/java/com/palantir/javapoet/FieldSpec.java
        /*
    * Copyright (C) 2015 Square, Inc.
    *
    * Licensed under the Apache License, Version 2.0 (the "License");
    * you may not use this file except in compliance with the License.
    * You may obtain a copy of the License at
    *
    * http://www.apache.org/licenses/LICENSE-2.0
    *
    * Unless required by applicable law or agreed to in writing, software
    * distributed under the License is distributed on an "AS IS" BASIS,
    * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    * See the License for the specific language governing permissions and
    * limitations under the License.
    */
    package com.palantir.javapoet;
    import java.io.IOException;
    import java.io.UncheckedIOException;
    import java.lang.reflect.Type;
    import java.util.ArrayList;
    import java.util.Collections;
    import java.util.List;
    import java.util.Set;
    import javax.lang.model.SourceVersion;
    import javax.lang.model.element.Modifier;

    /** A generated field declaration. */
    public final class FieldSpec {
        private final TypeName type;
        private final String name;
        private final List<AnnotationSpec> annotations;
        private final Set<Modifier> modifiers;
        private final CodeBlock initializer;

    private FieldSpec(Builder builder) {
        this.type = checkNotNull(builder.type, "type == null");
        this.name = checkNotNull(builder.name, "name == null");
        this.javadoc = builder.javadoc.build();
        this.annotations = Util.immutableList(builder.annotations);
        this.modifiers = Util.immutableSet(builder.modifiers);
        this.initializer = (builder.initializer == null) ? CodeBlock.builder().build() : builder.initializer;
    }
}