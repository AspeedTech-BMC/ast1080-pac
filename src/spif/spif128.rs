#[doc = "Register `SPIF128` reader"]
pub type R = crate::R<Spif128Spec>;
#[doc = "Register `SPIF128` writer"]
pub type W = crate::W<Spif128Spec>;
#[doc = "Field `ADDRCTL10` reader - ADDR_CTL10"]
pub type Addrctl10R = crate::FieldReader;
#[doc = "Field `ADDRCTL10` writer - ADDR_CTL10"]
pub type Addrctl10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL10"]
    #[inline(always)]
    pub fn addrctl10(&self) -> Addrctl10R {
        Addrctl10R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL10"]
    #[inline(always)]
    pub fn addrctl10(&mut self) -> Addrctl10W<Spif128Spec> {
        Addrctl10W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif128Spec;
impl crate::RegisterSpec for Spif128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif128::R`](R) reader structure"]
impl crate::Readable for Spif128Spec {}
#[doc = "`write(|w| ..)` method takes [`spif128::W`](W) writer structure"]
impl crate::Writable for Spif128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF128 to value 0"]
impl crate::Resettable for Spif128Spec {}
