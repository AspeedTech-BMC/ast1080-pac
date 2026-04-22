#[doc = "Register `SPIF12C` reader"]
pub type R = crate::R<Spif12cSpec>;
#[doc = "Register `SPIF12C` writer"]
pub type W = crate::W<Spif12cSpec>;
#[doc = "Field `ADDRCTL11` reader - ADDR_CTL11"]
pub type Addrctl11R = crate::FieldReader;
#[doc = "Field `ADDRCTL11` writer - ADDR_CTL11"]
pub type Addrctl11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL11"]
    #[inline(always)]
    pub fn addrctl11(&self) -> Addrctl11R {
        Addrctl11R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL11"]
    #[inline(always)]
    pub fn addrctl11(&mut self) -> Addrctl11W<Spif12cSpec> {
        Addrctl11W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif12cSpec;
impl crate::RegisterSpec for Spif12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif12c::R`](R) reader structure"]
impl crate::Readable for Spif12cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif12c::W`](W) writer structure"]
impl crate::Writable for Spif12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF12C to value 0"]
impl crate::Resettable for Spif12cSpec {}
