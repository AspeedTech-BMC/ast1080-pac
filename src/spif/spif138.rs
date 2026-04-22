#[doc = "Register `SPIF138` reader"]
pub type R = crate::R<Spif138Spec>;
#[doc = "Register `SPIF138` writer"]
pub type W = crate::W<Spif138Spec>;
#[doc = "Field `ADDRCTL14` reader - ADDR_CTL14"]
pub type Addrctl14R = crate::FieldReader;
#[doc = "Field `ADDRCTL14` writer - ADDR_CTL14"]
pub type Addrctl14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL14"]
    #[inline(always)]
    pub fn addrctl14(&self) -> Addrctl14R {
        Addrctl14R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL14"]
    #[inline(always)]
    pub fn addrctl14(&mut self) -> Addrctl14W<Spif138Spec> {
        Addrctl14W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL14\n\nYou can [`read`](crate::Reg::read) this register and get [`spif138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif138Spec;
impl crate::RegisterSpec for Spif138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif138::R`](R) reader structure"]
impl crate::Readable for Spif138Spec {}
#[doc = "`write(|w| ..)` method takes [`spif138::W`](W) writer structure"]
impl crate::Writable for Spif138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF138 to value 0"]
impl crate::Resettable for Spif138Spec {}
