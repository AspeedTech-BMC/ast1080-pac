#[doc = "Register `SPIF13C` reader"]
pub type R = crate::R<Spif13cSpec>;
#[doc = "Register `SPIF13C` writer"]
pub type W = crate::W<Spif13cSpec>;
#[doc = "Field `ADDRCTL15` reader - ADDR_CTL15"]
pub type Addrctl15R = crate::FieldReader;
#[doc = "Field `ADDRCTL15` writer - ADDR_CTL15"]
pub type Addrctl15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL15"]
    #[inline(always)]
    pub fn addrctl15(&self) -> Addrctl15R {
        Addrctl15R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL15"]
    #[inline(always)]
    pub fn addrctl15(&mut self) -> Addrctl15W<Spif13cSpec> {
        Addrctl15W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif13cSpec;
impl crate::RegisterSpec for Spif13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif13c::R`](R) reader structure"]
impl crate::Readable for Spif13cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif13c::W`](W) writer structure"]
impl crate::Writable for Spif13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF13C to value 0"]
impl crate::Resettable for Spif13cSpec {}
