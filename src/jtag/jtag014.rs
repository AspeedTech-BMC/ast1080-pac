#[doc = "Register `JTAG014` reader"]
pub type R = crate::R<Jtag014Spec>;
#[doc = "Register `JTAG014` writer"]
pub type W = crate::W<Jtag014Spec>;
#[doc = "Field `TCKDivisor` reader - TCK divisor."]
pub type TckdivisorR = crate::FieldReader<u16>;
#[doc = "Field `TCKDivisor` writer - TCK divisor."]
pub type TckdivisorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "TCK inverse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tckinverse {
    #[doc = "1: Inverse TCK."]
    InverseTck = 1,
    #[doc = "0: Not inverse TCK."]
    NotInverseTck = 0,
}
impl From<Tckinverse> for bool {
    #[inline(always)]
    fn from(variant: Tckinverse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCKInverse` reader - TCK inverse."]
pub type TckinverseR = crate::BitReader<Tckinverse>;
impl TckinverseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tckinverse {
        match self.bits {
            true => Tckinverse::InverseTck,
            false => Tckinverse::NotInverseTck,
        }
    }
    #[doc = "Inverse TCK."]
    #[inline(always)]
    pub fn is_inverse_tck(&self) -> bool {
        *self == Tckinverse::InverseTck
    }
    #[doc = "Not inverse TCK."]
    #[inline(always)]
    pub fn is_not_inverse_tck(&self) -> bool {
        *self == Tckinverse::NotInverseTck
    }
}
#[doc = "Field `TCKInverse` writer - TCK inverse."]
pub type TckinverseW<'a, REG> = crate::BitWriter<'a, REG, Tckinverse>;
impl<'a, REG> TckinverseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverse TCK."]
    #[inline(always)]
    pub fn inverse_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Tckinverse::InverseTck)
    }
    #[doc = "Not inverse TCK."]
    #[inline(always)]
    pub fn not_inverse_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Tckinverse::NotInverseTck)
    }
}
impl R {
    #[doc = "Bits 0:10 - TCK divisor."]
    #[inline(always)]
    pub fn tckdivisor(&self) -> TckdivisorR {
        TckdivisorR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - TCK inverse."]
    #[inline(always)]
    pub fn tckinverse(&self) -> TckinverseR {
        TckinverseR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - TCK divisor."]
    #[inline(always)]
    pub fn tckdivisor(&mut self) -> TckdivisorW<Jtag014Spec> {
        TckdivisorW::new(self, 0)
    }
    #[doc = "Bit 31 - TCK inverse."]
    #[inline(always)]
    pub fn tckinverse(&mut self) -> TckinverseW<Jtag014Spec> {
        TckinverseW::new(self, 31)
    }
}
#[doc = "TCK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag014Spec;
impl crate::RegisterSpec for Jtag014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag014::R`](R) reader structure"]
impl crate::Readable for Jtag014Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag014::W`](W) writer structure"]
impl crate::Writable for Jtag014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG014 to value 0x07"]
impl crate::Resettable for Jtag014Spec {
    const RESET_VALUE: u32 = 0x07;
}
