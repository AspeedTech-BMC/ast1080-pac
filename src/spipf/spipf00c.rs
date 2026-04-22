#[doc = "Register `SPIPF00C` reader"]
pub type R = crate::R<Spipf00cSpec>;
#[doc = "Register `SPIPF00C` writer"]
pub type W = crate::W<Spipf00cSpec>;
#[doc = "Field `ReadDataOfBlockFIFO` reader - Read Data of Block FIFO"]
pub type ReadDataOfBlockFifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data of Block FIFO"]
    #[inline(always)]
    pub fn read_data_of_block_fifo(&self) -> ReadDataOfBlockFifoR {
        ReadDataOfBlockFifoR::new(self.bits)
    }
}
impl W {}
#[doc = "Block FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf00cSpec;
impl crate::RegisterSpec for Spipf00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf00c::R`](R) reader structure"]
impl crate::Readable for Spipf00cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf00c::W`](W) writer structure"]
impl crate::Writable for Spipf00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF00C to value 0"]
impl crate::Resettable for Spipf00cSpec {}
