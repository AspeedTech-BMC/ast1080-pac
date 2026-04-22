#[doc = "Register `SPIF0EC` reader"]
pub type R = crate::R<Spif0ecSpec>;
#[doc = "Register `SPIF0EC` writer"]
pub type W = crate::W<Spif0ecSpec>;
#[doc = "Field `WTABLE27` reader - WTABLE27"]
pub type Wtable27R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE27` writer - WTABLE27"]
pub type Wtable27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE27"]
    #[inline(always)]
    pub fn wtable27(&self) -> Wtable27R {
        Wtable27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE27"]
    #[inline(always)]
    pub fn wtable27(&mut self) -> Wtable27W<Spif0ecSpec> {
        Wtable27W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE27\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0ecSpec;
impl crate::RegisterSpec for Spif0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0ec::R`](R) reader structure"]
impl crate::Readable for Spif0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0ec::W`](W) writer structure"]
impl crate::Writable for Spif0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0EC to value 0"]
impl crate::Resettable for Spif0ecSpec {}
