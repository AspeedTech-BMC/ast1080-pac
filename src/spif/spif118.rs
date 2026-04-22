#[doc = "Register `SPIF118` reader"]
pub type R = crate::R<Spif118Spec>;
#[doc = "Register `SPIF118` writer"]
pub type W = crate::W<Spif118Spec>;
#[doc = "Field `ADDRCTL06` reader - ADDR_CTL06"]
pub type Addrctl06R = crate::FieldReader;
#[doc = "Field `ADDRCTL06` writer - ADDR_CTL06"]
pub type Addrctl06W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL06"]
    #[inline(always)]
    pub fn addrctl06(&self) -> Addrctl06R {
        Addrctl06R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL06"]
    #[inline(always)]
    pub fn addrctl06(&mut self) -> Addrctl06W<Spif118Spec> {
        Addrctl06W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif118Spec;
impl crate::RegisterSpec for Spif118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif118::R`](R) reader structure"]
impl crate::Readable for Spif118Spec {}
#[doc = "`write(|w| ..)` method takes [`spif118::W`](W) writer structure"]
impl crate::Writable for Spif118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF118 to value 0"]
impl crate::Resettable for Spif118Spec {}
