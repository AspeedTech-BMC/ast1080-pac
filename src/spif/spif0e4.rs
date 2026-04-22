#[doc = "Register `SPIF0E4` reader"]
pub type R = crate::R<Spif0e4Spec>;
#[doc = "Register `SPIF0E4` writer"]
pub type W = crate::W<Spif0e4Spec>;
#[doc = "Field `WTABLE25` reader - WTABLE25"]
pub type Wtable25R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE25` writer - WTABLE25"]
pub type Wtable25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE25"]
    #[inline(always)]
    pub fn wtable25(&self) -> Wtable25R {
        Wtable25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE25"]
    #[inline(always)]
    pub fn wtable25(&mut self) -> Wtable25W<Spif0e4Spec> {
        Wtable25W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE25\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0e4Spec;
impl crate::RegisterSpec for Spif0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0e4::R`](R) reader structure"]
impl crate::Readable for Spif0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0e4::W`](W) writer structure"]
impl crate::Writable for Spif0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0E4 to value 0"]
impl crate::Resettable for Spif0e4Spec {}
