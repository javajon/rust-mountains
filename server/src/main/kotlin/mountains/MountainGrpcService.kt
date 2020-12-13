package mountains

import org.lognet.springboot.grpc.GRpcService
import java.util.UUID

@GRpcService
class MountainGrpcService: MountainServiceGrpcKt.MountainServiceCoroutineImplBase() {

    /** The "domain model" is simply a mutable list of Peaks for demonstration. */
    var peaks = mutableListOf(
        Peak("a48a6488-fba6-11ea-adc1-0242ac120002",
            "Mt. Washington",
            1917,
            "44_16_13.8_N_71_18_11.7_W"),
        Peak(
            "a48a8cce-fba6-11ea-adc1-0242ac120002",
            "Flatirons",
            2484,
            "39.988_N_105.293_W")
    )

    /** Convert from domain rep of a mountain to a GRPC mountain object. */
    fun genMountain(peak: Peak): Mountains.Mountain {
        return Mountains.Mountain.newBuilder().
            setId(peak.id).
            setName(peak.name).
            setElevation(peak.elevation).
            setLocation(peak.location).build()
    }

    /** GRPC GetAll */
    override suspend fun getAll(request: Mountains.Empty): Mountains.MountainList {
        val grpcMountains = Mountains.MountainList.newBuilder()

        for (peak in peaks) {
            grpcMountains.addMountains(genMountain(peak))
        }

        return grpcMountains.build()
    }

    /** GRPC Get */
    override suspend fun get(request: Mountains.MountainRequestId): Mountains.Mountain {
        return genMountain(peaks.first { e -> e.id == request.id })
    }

    /** GRPC Insert */
    override suspend fun insert(request: Mountains.Mountain): Mountains.Mountain {
        peaks.add(Peak(UUID.randomUUID().toString(), request.name, request.elevation, request.location))
        return request
    }

    /** GRPC Update */
    override suspend fun update(request: Mountains.Mountain): Mountains.Mountain {
        val index = peaks.indexOfFirst { e -> e.id == request.id }
        peaks.set(index, Peak(request.id, request.name, request.elevation, request.location))
        return request
    }

    /** GRPC Remove */
    override suspend fun remove(request: Mountains.MountainRequestId): Mountains.Empty {
        peaks.removeAt(peaks.indexOfFirst { e -> e.id == request.id })
        return Mountains.Empty.getDefaultInstance()
    }
}
