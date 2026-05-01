#[doc = "Register `GPIOB20` reader"]
pub type R = crate::R<Gpiob20Spec>;
#[doc = "Register `GPIOB20` writer"]
pub type W = crate::W<Gpiob20Spec>;
#[doc = "GPIO128 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio128wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO128 is reset by WDT."]
    WritePrivilegeOfGpio128IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO128 is NOT reset by WDT."]
    WritePrivilegeOfGpio128IsNotResetByWdt = 1,
}
impl From<Gpio128wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio128wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO128WrPrivilegeRstTolerance` reader - GPIO128 Write Privilege Reset Tolerance"]
pub type Gpio128wrPrivilegeRstToleranceR = crate::BitReader<Gpio128wrPrivilegeRstTolerance>;
impl Gpio128wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio128wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsResetByWdt,
            true => Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO128 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio128_is_reset_by_wdt(&self) -> bool {
        *self == Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO128 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio128_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsNotResetByWdt
    }
}
#[doc = "Field `GPIO128WrPrivilegeRstTolerance` writer - GPIO128 Write Privilege Reset Tolerance"]
pub type Gpio128wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio128wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio128wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO128 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio128_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO128 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio128_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128wrPrivilegeRstTolerance::WritePrivilegeOfGpio128IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO129WrPrivilegeRstTolerance` reader - GPIO129 Write Privilege Reset Tolerance"]
pub type Gpio129wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO129WrPrivilegeRstTolerance` writer - GPIO129 Write Privilege Reset Tolerance"]
pub type Gpio129wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO130WrPrivilegeRstTolerance` reader - GPIO130 Write Privilege Reset Tolerance"]
pub type Gpio130wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO130WrPrivilegeRstTolerance` writer - GPIO130 Write Privilege Reset Tolerance"]
pub type Gpio130wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO131WrPrivilegeRstTolerance` reader - GPIO131 Write Privilege Reset Tolerance"]
pub type Gpio131wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO131WrPrivilegeRstTolerance` writer - GPIO131 Write Privilege Reset Tolerance"]
pub type Gpio131wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO132WrPrivilegeRstTolerance` reader - GPIO132 Write Privilege Reset Tolerance"]
pub type Gpio132wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO132WrPrivilegeRstTolerance` writer - GPIO132 Write Privilege Reset Tolerance"]
pub type Gpio132wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO133WrPrivilegeRstTolerance` reader - GPIO133 Write Privilege Reset Tolerance"]
pub type Gpio133wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO133WrPrivilegeRstTolerance` writer - GPIO133 Write Privilege Reset Tolerance"]
pub type Gpio133wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO134WrPrivilegeRstTolerance` reader - GPIO134 Write Privilege Reset Tolerance"]
pub type Gpio134wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO134WrPrivilegeRstTolerance` writer - GPIO134 Write Privilege Reset Tolerance"]
pub type Gpio134wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO135WrPrivilegeRstTolerance` reader - GPIO135 Write Privilege Reset Tolerance"]
pub type Gpio135wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO135WrPrivilegeRstTolerance` writer - GPIO135 Write Privilege Reset Tolerance"]
pub type Gpio135wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO136WrPrivilegeRstTolerance` reader - GPIO136 Write Privilege Reset Tolerance"]
pub type Gpio136wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO136WrPrivilegeRstTolerance` writer - GPIO136 Write Privilege Reset Tolerance"]
pub type Gpio136wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO137WrPrivilegeRstTolerance` reader - GPIO137 Write Privilege Reset Tolerance"]
pub type Gpio137wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO137WrPrivilegeRstTolerance` writer - GPIO137 Write Privilege Reset Tolerance"]
pub type Gpio137wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO138WrPrivilegeRstTolerance` reader - GPIO138 Write Privilege Reset Tolerance"]
pub type Gpio138wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO138WrPrivilegeRstTolerance` writer - GPIO138 Write Privilege Reset Tolerance"]
pub type Gpio138wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO139WrPrivilegeRstTolerance` reader - GPIO139 Write Privilege Reset Tolerance"]
pub type Gpio139wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO139WrPrivilegeRstTolerance` writer - GPIO139 Write Privilege Reset Tolerance"]
pub type Gpio139wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO140WrPrivilegeRstTolerance` reader - GPIO140 Write Privilege Reset Tolerance"]
pub type Gpio140wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO140WrPrivilegeRstTolerance` writer - GPIO140 Write Privilege Reset Tolerance"]
pub type Gpio140wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO141WrPrivilegeRstTolerance` reader - GPIO141 Write Privilege Reset Tolerance"]
pub type Gpio141wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO141WrPrivilegeRstTolerance` writer - GPIO141 Write Privilege Reset Tolerance"]
pub type Gpio141wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO142WrPrivilegeRstTolerance` reader - GPIO142 Write Privilege Reset Tolerance"]
pub type Gpio142wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO142WrPrivilegeRstTolerance` writer - GPIO142 Write Privilege Reset Tolerance"]
pub type Gpio142wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO143WrPrivilegeRstTolerance` reader - GPIO143 Write Privilege Reset Tolerance"]
pub type Gpio143wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO143WrPrivilegeRstTolerance` writer - GPIO143 Write Privilege Reset Tolerance"]
pub type Gpio143wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO144WrPrivilegeRstTolerance` reader - GPIO144 Write Privilege Reset Tolerance"]
pub type Gpio144wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO144WrPrivilegeRstTolerance` writer - GPIO144 Write Privilege Reset Tolerance"]
pub type Gpio144wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO145WrPrivilegeRstTolerance` reader - GPIO145 Write Privilege Reset Tolerance"]
pub type Gpio145wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO145WrPrivilegeRstTolerance` writer - GPIO145 Write Privilege Reset Tolerance"]
pub type Gpio145wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO146WrPrivilegeRstTolerance` reader - GPIO146 Write Privilege Reset Tolerance"]
pub type Gpio146wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO146WrPrivilegeRstTolerance` writer - GPIO146 Write Privilege Reset Tolerance"]
pub type Gpio146wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO147WrPrivilegeRstTolerance` reader - GPIO147 Write Privilege Reset Tolerance"]
pub type Gpio147wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO147WrPrivilegeRstTolerance` writer - GPIO147 Write Privilege Reset Tolerance"]
pub type Gpio147wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO148WrPrivilegeRstTolerance` reader - GPIO148 Write Privilege Reset Tolerance"]
pub type Gpio148wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO148WrPrivilegeRstTolerance` writer - GPIO148 Write Privilege Reset Tolerance"]
pub type Gpio148wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO149WrPrivilegeRstTolerance` reader - GPIO149 Write Privilege Reset Tolerance"]
pub type Gpio149wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO149WrPrivilegeRstTolerance` writer - GPIO149 Write Privilege Reset Tolerance"]
pub type Gpio149wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO150WrPrivilegeRstTolerance` reader - GPIO150 Write Privilege Reset Tolerance"]
pub type Gpio150wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO150WrPrivilegeRstTolerance` writer - GPIO150 Write Privilege Reset Tolerance"]
pub type Gpio150wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO151WrPrivilegeRstTolerance` reader - GPIO151 Write Privilege Reset Tolerance"]
pub type Gpio151wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO151WrPrivilegeRstTolerance` writer - GPIO151 Write Privilege Reset Tolerance"]
pub type Gpio151wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO152WrPrivilegeRstTolerance` reader - GPIO152 Write Privilege Reset Tolerance"]
pub type Gpio152wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO152WrPrivilegeRstTolerance` writer - GPIO152 Write Privilege Reset Tolerance"]
pub type Gpio152wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO153WrPrivilegeRstTolerance` reader - GPIO153 Write Privilege Reset Tolerance"]
pub type Gpio153wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO153WrPrivilegeRstTolerance` writer - GPIO153 Write Privilege Reset Tolerance"]
pub type Gpio153wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO154WrPrivilegeRstTolerance` reader - GPIO154 Write Privilege Reset Tolerance"]
pub type Gpio154wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO154WrPrivilegeRstTolerance` writer - GPIO154 Write Privilege Reset Tolerance"]
pub type Gpio154wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO155WrPrivilegeRstTolerance` reader - GPIO155 Write Privilege Reset Tolerance"]
pub type Gpio155wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO155WrPrivilegeRstTolerance` writer - GPIO155 Write Privilege Reset Tolerance"]
pub type Gpio155wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO156WrPrivilegeRstTolerance` reader - GPIO156 Write Privilege Reset Tolerance"]
pub type Gpio156wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO156WrPrivilegeRstTolerance` writer - GPIO156 Write Privilege Reset Tolerance"]
pub type Gpio156wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO157WrPrivilegeRstTolerance` reader - GPIO157 Write Privilege Reset Tolerance"]
pub type Gpio157wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO157WrPrivilegeRstTolerance` writer - GPIO157 Write Privilege Reset Tolerance"]
pub type Gpio157wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO158WrPrivilegeRstTolerance` reader - GPIO158 Write Privilege Reset Tolerance"]
pub type Gpio158wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO158WrPrivilegeRstTolerance` writer - GPIO158 Write Privilege Reset Tolerance"]
pub type Gpio158wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO159WrPrivilegeRstTolerance` reader - GPIO159 Write Privilege Reset Tolerance"]
pub type Gpio159wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO159WrPrivilegeRstTolerance` writer - GPIO159 Write Privilege Reset Tolerance"]
pub type Gpio159wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO128 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128wr_privilege_rst_tolerance(&self) -> Gpio128wrPrivilegeRstToleranceR {
        Gpio128wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO129 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129wr_privilege_rst_tolerance(&self) -> Gpio129wrPrivilegeRstToleranceR {
        Gpio129wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO130 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130wr_privilege_rst_tolerance(&self) -> Gpio130wrPrivilegeRstToleranceR {
        Gpio130wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO131 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131wr_privilege_rst_tolerance(&self) -> Gpio131wrPrivilegeRstToleranceR {
        Gpio131wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO132 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132wr_privilege_rst_tolerance(&self) -> Gpio132wrPrivilegeRstToleranceR {
        Gpio132wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO133 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133wr_privilege_rst_tolerance(&self) -> Gpio133wrPrivilegeRstToleranceR {
        Gpio133wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO134 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134wr_privilege_rst_tolerance(&self) -> Gpio134wrPrivilegeRstToleranceR {
        Gpio134wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO135 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135wr_privilege_rst_tolerance(&self) -> Gpio135wrPrivilegeRstToleranceR {
        Gpio135wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO136 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136wr_privilege_rst_tolerance(&self) -> Gpio136wrPrivilegeRstToleranceR {
        Gpio136wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO137 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137wr_privilege_rst_tolerance(&self) -> Gpio137wrPrivilegeRstToleranceR {
        Gpio137wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO138 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138wr_privilege_rst_tolerance(&self) -> Gpio138wrPrivilegeRstToleranceR {
        Gpio138wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO139 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139wr_privilege_rst_tolerance(&self) -> Gpio139wrPrivilegeRstToleranceR {
        Gpio139wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO140 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140wr_privilege_rst_tolerance(&self) -> Gpio140wrPrivilegeRstToleranceR {
        Gpio140wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO141 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141wr_privilege_rst_tolerance(&self) -> Gpio141wrPrivilegeRstToleranceR {
        Gpio141wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO142 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142wr_privilege_rst_tolerance(&self) -> Gpio142wrPrivilegeRstToleranceR {
        Gpio142wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO143 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143wr_privilege_rst_tolerance(&self) -> Gpio143wrPrivilegeRstToleranceR {
        Gpio143wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO144 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144wr_privilege_rst_tolerance(&self) -> Gpio144wrPrivilegeRstToleranceR {
        Gpio144wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO145 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145wr_privilege_rst_tolerance(&self) -> Gpio145wrPrivilegeRstToleranceR {
        Gpio145wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO146 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146wr_privilege_rst_tolerance(&self) -> Gpio146wrPrivilegeRstToleranceR {
        Gpio146wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO147 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147wr_privilege_rst_tolerance(&self) -> Gpio147wrPrivilegeRstToleranceR {
        Gpio147wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO148 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148wr_privilege_rst_tolerance(&self) -> Gpio148wrPrivilegeRstToleranceR {
        Gpio148wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO149 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149wr_privilege_rst_tolerance(&self) -> Gpio149wrPrivilegeRstToleranceR {
        Gpio149wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO150 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150wr_privilege_rst_tolerance(&self) -> Gpio150wrPrivilegeRstToleranceR {
        Gpio150wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO151 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151wr_privilege_rst_tolerance(&self) -> Gpio151wrPrivilegeRstToleranceR {
        Gpio151wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO152 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152wr_privilege_rst_tolerance(&self) -> Gpio152wrPrivilegeRstToleranceR {
        Gpio152wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO153 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153wr_privilege_rst_tolerance(&self) -> Gpio153wrPrivilegeRstToleranceR {
        Gpio153wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO154 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154wr_privilege_rst_tolerance(&self) -> Gpio154wrPrivilegeRstToleranceR {
        Gpio154wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO155 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155wr_privilege_rst_tolerance(&self) -> Gpio155wrPrivilegeRstToleranceR {
        Gpio155wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO156 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156wr_privilege_rst_tolerance(&self) -> Gpio156wrPrivilegeRstToleranceR {
        Gpio156wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO157 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157wr_privilege_rst_tolerance(&self) -> Gpio157wrPrivilegeRstToleranceR {
        Gpio157wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO158 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158wr_privilege_rst_tolerance(&self) -> Gpio158wrPrivilegeRstToleranceR {
        Gpio158wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO159 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159wr_privilege_rst_tolerance(&self) -> Gpio159wrPrivilegeRstToleranceR {
        Gpio159wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO128 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio128wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio128wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO129 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio129wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio129wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO130 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio130wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio130wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO131 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio131wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio131wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO132 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio132wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio132wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO133 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio133wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio133wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO134 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio134wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio134wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO135 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio135wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio135wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO136 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio136wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio136wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO137 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio137wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio137wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO138 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio138wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio138wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO139 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio139wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio139wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO140 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio140wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio140wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO141 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio141wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio141wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO142 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio142wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio142wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO143 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio143wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio143wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO144 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio144wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio144wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO145 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio145wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio145wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO146 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio146wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio146wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO147 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio147wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio147wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO148 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio148wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio148wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO149 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio149wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio149wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO150 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio150wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio150wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO151 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio151wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio151wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO152 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio152wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio152wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO153 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio153wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio153wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO154 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio154wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio154wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO155 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio155wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio155wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO156 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio156wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio156wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO157 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio157wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio157wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO158 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio158wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio158wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO159 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio159wrPrivilegeRstToleranceW<Gpiob20Spec> {
        Gpio159wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob20Spec;
impl crate::RegisterSpec for Gpiob20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob20::R`](R) reader structure"]
impl crate::Readable for Gpiob20Spec {}
#[doc = "`write(|w| ..)` method takes [`gpiob20::W`](W) writer structure"]
impl crate::Writable for Gpiob20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB20 to value 0"]
impl crate::Resettable for Gpiob20Spec {}
