# System Design Questions

This folder contains a comprehensive set of system design interview questions, segmented by topic and difficulty. Use these to practice, prepare, or guide discussions for intermediate to expert-level system design interviews.

## General Guidelines for System Design Solutions

For every system design question, address the following:

1. **Requirements & Constraints**
   - Functional and non-functional requirements (scale, latency, availability, durability, cost)
   - Estimate scale (users, QPS, data size, growth rate)
2. **High-Level Architecture**
   - Request flow (client → load balancer → app servers → DB → cache → storage)
3. **Data Model & Storage**
   - Relational vs NoSQL, sharding, partitioning, replication, indexing
4. **Scalability Considerations**
   - Horizontal/vertical scaling, caching (CDN, Redis, Memcached)
5. **Reliability & Fault Tolerance**
   - Failover, replication, backups, CAP theorem
6. **Performance Optimizations**
   - Caching strategies, query optimization, batching, async/event-driven
7. **Security & Compliance**
   - Authentication, authorization, rate limiting, data privacy, encryption
8. **Trade-offs**
   - SQL vs NoSQL, consistency vs availability, monolith vs microservices

---

## 📚 System Design Question List

### 🌐 Web / Application Layer
1. Design a URL Shortener (like bit.ly)
2. Design a Rate Limiter for an API Gateway
3. Design a Real-Time Chat System (like WhatsApp)
4. Design a News Feed System (like Facebook)
5. Design a Notification System (push/email/SMS)
6. Design an API Gateway (throttling, routing, auth)
7. Design a Video Streaming Service (like YouTube)
8. Design a Real-Time Collaboration Tool (like Google Docs)
9. Design a Web Crawler & Indexer (like Google Search)
10. Design a Content Delivery Network (CDN)

### 💾 Database & Storage Layer
11. Design a Key-Value Store (like Redis)
12. Design a Distributed Cache with eviction policy
13. Design a SQL Sharding Strategy for a growing e-commerce DB
14. Design a Time-Series Database (like InfluxDB)
15. Design a Distributed File Storage System (like HDFS, S3)
16. Design a Search System with indexing (like Elasticsearch)
17. Design a Multi-tenant Database for SaaS apps
18. Design a Data Warehouse for analytics
19. Design an Event Sourcing System with CQRS
20. Design a Replication & Backup Strategy for a global DB

### 🚦 Scalability & Reliability
21. Design a Load Balancer (L4 vs L7)
22. Design a Distributed Queue System (like Kafka)
23. Design a Job Scheduler & Worker System
24. Design a Rate-Adaptive Streaming System (video quality adapts to bandwidth)
25. Design a Global User Authentication System (OAuth, SSO)
26. Design a Multi-Region Active-Active Deployment
27. Design a High-Availability DNS System
28. Design a Monitoring & Metrics Platform (like Prometheus)
29. Design a Logging & Aggregation System (like ELK stack)
30. Design a Disaster Recovery Plan for a large SaaS service

### 🔥 Advanced / Expert Level
31. Design a Payment System (like Stripe)
32. Design a Recommendation System (like Netflix)
33. Design a Ride-Sharing System (like Uber)
34. Design an E-commerce Platform (like Amazon)
35. Design a Microservices Communication Layer (RPC, gRPC, messaging)
36. Design a Multi-Region Data Sync Service (eventual consistency)
37. Design a Graph Database (like Neo4j)
38. Design a Blockchain-based Ledger System
39. Design a Machine Learning Feature Store
40. **Capstone:** Design a Global-Scale Social Media Platform (Instagram/Twitter) with feeds, stories, DMs, notifications, scaling, sharding, caching, observability, abuse prevention

---

Feel free to contribute solutions, diagrams, or improvements!
