package com.example.kotlin.config

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.context.annotation.Primary
import org.springframework.web.reactive.function.client.WebClient

@Configuration
class HttpConfig {
    @Bean
    @Primary
    fun httpClient(): WebClient = WebClient.builder().codecs { config -> config.defaultCodecs().maxInMemorySize(16 * 1024 * 1024) }.build()
}