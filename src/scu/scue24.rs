#[doc = "Register `SCUE24` reader"]
pub type R = crate::R<Scue24Spec>;
#[doc = "Register `SCUE24` writer"]
pub type W = crate::W<Scue24Spec>;
#[doc = "Field `SCUREGLOCK4A0` reader - SCU_REG_LOCK_4A0"]
pub type Scureglock4a0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK4A0` writer - SCU_REG_LOCK_4A0"]
pub type Scureglock4a0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - SCU_REG_LOCK_4A0"]
    #[inline(always)]
    pub fn scureglock4a0(&self) -> Scureglock4a0R {
        Scureglock4a0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SCU_REG_LOCK_4A0"]
    #[inline(always)]
    pub fn scureglock4a0(&mut self) -> Scureglock4a0W<Scue24Spec> {
        Scureglock4a0W::new(self, 8)
    }
}
#[doc = "Write Protection 10 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue24Spec;
impl crate::RegisterSpec for Scue24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue24::R`](R) reader structure"]
impl crate::Readable for Scue24Spec {}
#[doc = "`write(|w| ..)` method takes [`scue24::W`](W) writer structure"]
impl crate::Writable for Scue24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE24 to value 0"]
impl crate::Resettable for Scue24Spec {}
