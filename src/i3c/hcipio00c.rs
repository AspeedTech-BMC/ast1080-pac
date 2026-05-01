#[doc = "Register `HCIPIO00C` reader"]
pub type R = crate::R<Hcipio00cSpec>;
#[doc = "Register `HCIPIO00C` writer"]
pub type W = crate::W<Hcipio00cSpec>;
#[doc = "Field `REGIBIPORT` reader - REG_IBI_PORT"]
pub type RegibiportR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_IBI_PORT"]
    #[inline(always)]
    pub fn regibiport(&self) -> RegibiportR {
        RegibiportR::new(self.bits)
    }
}
impl W {}
#[doc = "IBI\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio00cSpec;
impl crate::RegisterSpec for Hcipio00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio00c::R`](R) reader structure"]
impl crate::Readable for Hcipio00cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcipio00c::W`](W) writer structure"]
impl crate::Writable for Hcipio00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO00C to value 0"]
impl crate::Resettable for Hcipio00cSpec {}
