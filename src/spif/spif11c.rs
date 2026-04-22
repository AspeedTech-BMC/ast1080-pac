#[doc = "Register `SPIF11C` reader"]
pub type R = crate::R<Spif11cSpec>;
#[doc = "Register `SPIF11C` writer"]
pub type W = crate::W<Spif11cSpec>;
#[doc = "Field `ADDRCTL07` reader - ADDR_CTL07"]
pub type Addrctl07R = crate::FieldReader;
#[doc = "Field `ADDRCTL07` writer - ADDR_CTL07"]
pub type Addrctl07W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL07"]
    #[inline(always)]
    pub fn addrctl07(&self) -> Addrctl07R {
        Addrctl07R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL07"]
    #[inline(always)]
    pub fn addrctl07(&mut self) -> Addrctl07W<Spif11cSpec> {
        Addrctl07W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif11cSpec;
impl crate::RegisterSpec for Spif11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif11c::R`](R) reader structure"]
impl crate::Readable for Spif11cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif11c::W`](W) writer structure"]
impl crate::Writable for Spif11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF11C to value 0"]
impl crate::Resettable for Spif11cSpec {}
