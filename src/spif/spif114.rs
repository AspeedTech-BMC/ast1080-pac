#[doc = "Register `SPIF114` reader"]
pub type R = crate::R<Spif114Spec>;
#[doc = "Register `SPIF114` writer"]
pub type W = crate::W<Spif114Spec>;
#[doc = "Field `ADDRCTL05` reader - ADDR_CTL05"]
pub type Addrctl05R = crate::FieldReader;
#[doc = "Field `ADDRCTL05` writer - ADDR_CTL05"]
pub type Addrctl05W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL05"]
    #[inline(always)]
    pub fn addrctl05(&self) -> Addrctl05R {
        Addrctl05R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL05"]
    #[inline(always)]
    pub fn addrctl05(&mut self) -> Addrctl05W<Spif114Spec> {
        Addrctl05W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif114Spec;
impl crate::RegisterSpec for Spif114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif114::R`](R) reader structure"]
impl crate::Readable for Spif114Spec {}
#[doc = "`write(|w| ..)` method takes [`spif114::W`](W) writer structure"]
impl crate::Writable for Spif114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF114 to value 0"]
impl crate::Resettable for Spif114Spec {}
